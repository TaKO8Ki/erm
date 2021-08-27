use crate::alias::create_alias;
use crate::archive::{self, extract::Error as ExtractError, extract::Extract};
use crate::config::FrumConfig;
use crate::input_version::InputVersion;
use crate::outln;
use crate::version::Version;
use crate::version_file::get_user_version_for_directory;
use anyhow::Result;
use colored::Colorize;
use log::debug;
use reqwest::Url;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrumError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Can't find the number of cores")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Can't extract the file: {source:?}")]
    ExtractError { source: ExtractError },
    #[error("The downloaded archive is empty")]
    TarIsEmpty,
    #[error("Can't find version: {version}")]
    VersionNotFound { version: InputVersion },
    #[error("Can't list the remote versions: {source:?}")]
    CantListRemoteVersions { source: reqwest::Error },
    #[error("Version already installed at {path:?}")]
    VersionAlreadyInstalled { path: PathBuf },
    #[error("Can't find version in dotfiles. Please provide a version manually to the command.")]
    CantInferVersion,
    #[error("The requested version is not installable: {version}")]
    NotInstallableVersion { version: Version },
    #[error("Can't build Ruby: {stderr}")]
    CantBuildRuby { stderr: String },
}

pub struct Install {
    pub version: Option<InputVersion>,
    pub configure_opts: Vec<String>,
}

impl crate::command::Command for Install {
    type Error = FrumError;

    fn apply(&self, config: &FrumConfig) -> Result<(), Self::Error> {
        let current_version = self
            .version
            .clone()
            .or_else(|| get_user_version_for_directory(std::env::current_dir().unwrap()))
            .ok_or(FrumError::CantInferVersion)?;
        let version = match current_version.clone() {
            InputVersion::Full(Version::Semver(v)) => Version::Semver(v),
            InputVersion::Full(Version::System) => {
                return Err(FrumError::NotInstallableVersion {
                    version: Version::System,
                })
            }
            current_version => {
                let available_versions = crate::remote_ruby_index::list(&config.ruby_build_mirror)
                    .map_err(|source| FrumError::CantListRemoteVersions { source })?
                    .drain(..)
                    .map(|x| x.version)
                    .collect::<Vec<_>>();

                current_version
                    .to_version(&available_versions)
                    .ok_or(FrumError::VersionNotFound {
                        version: current_version,
                    })?
                    .clone()
            }
        };
        let installations_dir = config.versions_dir();
        let installation_dir = PathBuf::from(&installations_dir).join(version.to_string());

        if installation_dir.exists() {
            return Err(FrumError::VersionAlreadyInstalled {
                path: installation_dir,
            });
        }

        let url = package_url(config.ruby_build_mirror.clone(), &version);
        outln!(config#Info, "{} Downloading {}", "==>".green(), format!("{}", url).green());
        let response = reqwest::blocking::get(url)?;
        if response.status() == 404 {
            return Err(FrumError::VersionNotFound {
                version: current_version,
            });
        }

        outln!(config#Info, "{} Extracting {}", "==>".green(), archive(&version).green());
        let temp_installations_dir = installations_dir.join(".downloads");
        std::fs::create_dir_all(&temp_installations_dir).map_err(FrumError::IoError)?;
        let temp_dir = tempfile::TempDir::new_in(&temp_installations_dir)
            .expect("Can't generate a temp directory");
        extract_archive_into(&temp_dir, response)?;

        outln!(config#Info, "{} Building {}", "==>".green(), format!("Ruby {}", current_version).green());
        let installed_directory = std::fs::read_dir(&temp_dir)
            .map_err(FrumError::IoError)?
            .next()
            .ok_or(FrumError::TarIsEmpty)?
            .map_err(FrumError::IoError)?;
        let installed_directory = installed_directory.path();
        build_package(
            &installed_directory,
            &installation_dir,
            &self.configure_opts,
        )?;

        if !config.default_version_dir().exists() {
            debug!("Use {} as the default version", current_version);
            create_alias(&config, "default", &version).map_err(FrumError::IoError)?;
        }
        Ok(())
    }
}

fn extract_archive_into<P: AsRef<Path>>(
    path: P,
    response: reqwest::blocking::Response,
) -> Result<(), FrumError> {
    #[cfg(unix)]
    let extractor = archive::tar_xz::TarXz::new(response);
    #[cfg(windows)]
    let extractor = archive::zip::Zip::new(response);
    extractor
        .extract_into(path)
        .map_err(|source| FrumError::ExtractError { source })?;
    Ok(())
}

fn package_url(mirror_url: Url, version: &Version) -> Url {
    debug!("pakage url");
    Url::parse(&format!(
        "{}/{}/{}",
        mirror_url.as_str().trim_end_matches('/'),
        match version {
            Version::Semver(version) => format!("{}.{}", version.major, version.minor),
            _ => unreachable!(),
        },
        archive(version),
    ))
    .unwrap()
}

#[cfg(unix)]
fn archive(version: &Version) -> String {
    format!("ruby-{}.tar.xz", version)
}

#[cfg(windows)]
fn archive(version: &Version) -> String {
    format!("ruby-{}.zip", version)
}

#[allow(clippy::unnecessary_wraps)]
fn openssl_dir() -> Result<String, FrumError> {
    #[cfg(target_os = "macos")]
    return Ok(String::from_utf8_lossy(
        &Command::new("brew")
            .arg("--prefix")
            .arg("openssl")
            .output()
            .map_err(FrumError::IoError)?
            .stdout,
    )
    .trim()
    .to_string());
    #[cfg(not(target_os = "macos"))]
    return Ok("/usr/local".to_string());
}

fn build_package(
    current_dir: &Path,
    installed_dir: &Path,
    configure_opts: &[String],
) -> Result<(), FrumError> {
    debug!("./configure {}", configure_opts.join(" "));
    let mut command = Command::new("sh");
    command
        .arg("configure")
        .arg(format!("--prefix={}", installed_dir.to_str().unwrap()))
        .args(configure_opts);

    // Provide a default value for --with-openssl-dir
    if !configure_opts
        .iter()
        .any(|opt| opt.starts_with("--with-openssl-dir"))
    {
        command.arg(format!("--with-openssl-dir={}", openssl_dir()?));
    }

    let configure = command
        .current_dir(&current_dir)
        .output()
        .map_err(FrumError::IoError)?;
    if !configure.status.success() {
        return Err(FrumError::CantBuildRuby {
            stderr: format!(
                "configure failed: {}",
                String::from_utf8_lossy(&configure.stderr).to_string()
            ),
        });
    };
    debug!("make -j {}", num_cpus::get().to_string());
    let make = Command::new("make")
        .arg("-j")
        .arg(num_cpus::get().to_string())
        .current_dir(&current_dir)
        .output()
        .map_err(FrumError::IoError)?;
    if !make.status.success() {
        return Err(FrumError::CantBuildRuby {
            stderr: format!(
                "make failed: {}",
                String::from_utf8_lossy(&make.stderr).to_string()
            ),
        });
    };
    debug!("make install");
    let make_install = Command::new("make")
        .arg("-j")
        .arg(&num_cpus::get().to_string())
        .arg("install")
        .current_dir(&current_dir)
        .output()
        .map_err(FrumError::IoError)?;
    if !make_install.status.success() {
        return Err(FrumError::CantBuildRuby {
            stderr: format!(
                "make install: {}",
                String::from_utf8_lossy(&make_install.stderr).to_string()
            ),
        });
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::config::FrumConfig;
    use crate::version::Version;
    use tempfile::tempdir;

    #[test]
    fn test_install_second_version() {
        let config = FrumConfig {
            base_dir: Some(tempdir().unwrap().path().to_path_buf()),
            ..Default::default()
        };

        Install {
            version: Some(InputVersion::Full(Version::Semver(
                semver::Version::parse("2.7.0").unwrap(),
            ))),
            configure_opts: vec![],
        }
        .apply(&config)
        .expect("Can't install 2.7.0");

        Install {
            version: Some(InputVersion::Full(Version::Semver(
                semver::Version::parse("2.6.4").unwrap(),
            ))),
            configure_opts: vec![],
        }
        .apply(&config)
        .expect("Can't install 2.6.4");

        assert_eq!(
            std::fs::read_link(&config.default_version_dir())
                .unwrap()
                .components()
                .last(),
            Some(std::path::Component::Normal(std::ffi::OsStr::new("2.7.0")))
        );
    }

    #[test]
    fn test_install_default_version() {
        let config = FrumConfig {
            base_dir: Some(tempdir().unwrap().path().to_path_buf()),
            ..Default::default()
        };

        Install {
            version: Some(InputVersion::Full(Version::Semver(
                semver::Version::parse("2.6.4").unwrap(),
            ))),
            configure_opts: vec![],
        }
        .apply(&config)
        .expect("Can't install");

        assert!(config.versions_dir().join("2.6.4").exists());
        assert!(config
            .versions_dir()
            .join("2.6.4")
            .join("bin")
            .join("ruby")
            .exists());
        assert!(config.default_version_dir().exists());
    }
}
