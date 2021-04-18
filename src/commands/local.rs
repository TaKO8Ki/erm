use crate::input_version::InputVersion;
use crate::symlink::{create_symlink_dir, remove_symlink_dir};
use crate::version_file::get_user_version_for_directory;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("We can't find the necessary environment variables to replace the Ruby version.")]
    FarmPathNotFound,
    #[error("Requested version {version} is not currently installed")]
    VersionNotFound { version: InputVersion },
    #[error("Can't find version in dotfiles. Please provide a version manually to the command.")]
    CantInferVersion,
}

pub struct Local {
    pub version: Option<InputVersion>,
}

impl crate::command::Command for Local {
    type Error = FarmError;

    fn apply(&self, config: &crate::config::FarmConfig) -> Result<(), FarmError> {
        let current_version = match self.version.clone().ok_or_else(|| {
            match get_user_version_for_directory(std::env::current_dir().unwrap()) {
                Some(version) => Ok(version),
                None => {
                    replace_symlink(
                        &config.default_version_dir(),
                        &config
                            .farm_path
                            .clone()
                            .ok_or(FarmError::FarmPathNotFound)?,
                    )?;
                    Err(FarmError::CantInferVersion)
                }
            }
        }) {
            Ok(version) => version,
            Err(result) => result?,
        };
        debug!("Use {} as the current version", current_version);
        if !&config
            .versions_dir()
            .join(current_version.to_string())
            .exists()
        {
            return Err(FarmError::VersionNotFound {
                version: current_version,
            });
        }
        replace_symlink(
            &config.versions_dir().join(current_version.to_string()),
            &config
                .farm_path
                .clone()
                .ok_or(FarmError::FarmPathNotFound)?,
        )
        .map_err(FarmError::IoError)?;
        Ok(())
    }
}

fn replace_symlink(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    let symlink_deletion_result = remove_symlink_dir(&to);
    match create_symlink_dir(&from, &to) {
        ok @ Ok(_) => ok,
        err @ Err(_) => symlink_deletion_result.and(err),
    }
}

#[cfg(test)]
mod tests {
    use super::{FarmError, Local};
    use crate::command::Command;
    use crate::config::FarmConfig;
    use crate::input_version::InputVersion;
    use crate::version::Version;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_local_specified_version() {
        let mut config = FarmConfig::default();
        config.base_dir = Some(tempdir().unwrap().path().to_path_buf());
        config.farm_path = Some(std::env::temp_dir().join(format!(
            "farm_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_millis(),
        )));
        let dir_path = config.versions_dir().join("2.6.4").join("bin");
        std::fs::create_dir_all(&dir_path).unwrap();
        File::create(dir_path.join("ruby")).unwrap();

        crate::commands::global::Global {
            version: InputVersion::Full(Version::Semver(semver::Version::parse("2.6.4").unwrap())),
        }
        .apply(&config)
        .unwrap();

        Local {
            version: Some(InputVersion::Full(Version::Semver(
                semver::Version::parse("2.6.4").unwrap(),
            ))),
        }
        .apply(&config)
        .expect("failed to install");

        assert!(config.farm_path.unwrap().join("bin").join("ruby").exists());
    }

    #[test]
    fn test_not_found_version() {
        let mut config = FarmConfig::default();
        config.farm_path = Some(std::env::temp_dir().join(format!(
            "farm_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_millis(),
        )));
        let result = Local {
            version: Some(InputVersion::Full(Version::Semver(
                semver::Version::parse("2.6.4").unwrap(),
            ))),
        }
        .apply(&config);
        match result {
            Ok(_) => assert!(false),
            Err(FarmError::VersionNotFound { .. }) => assert!(true),
            _ => assert!(false),
        }
    }
}
