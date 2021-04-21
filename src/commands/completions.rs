use crate::cli::build_cli;
use crate::command::Command;
use crate::config::FarmConfig;
use crate::outln;
use crate::shell::{infer_shell, AVAILABLE_SHELLS};
use crate::version::{current_version, is_dotfile, Version};
use clap::Shell;
use colored::Colorize;
use log::debug;
use thiserror::Error;

const USE_COMMAND_REGEX: &str = r#"opts=" -h -V  --help --version  "#;
const INSTALL_COMMAND_REGEX: &str =
    r#"opts=" -l -w -h -V  --list --with-openssl-dir --help --version  "#;
const UNINSTALL_COMMAND_REGEX: &str = r#"opts=" -h -V  --help --version  "#;

#[derive(Debug)]
enum FarmCommand {
    Install,
    Uninstall,
    Local,
    Global,
    None,
}

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(
        "{}\n{}\n{}\n{}",
        "Can't infer shell!",
        "fnm can't infer your shell based on the process tree.",
        "Maybe it is unsupported? we support the following shells:",
        shells_as_string()
    )]
    CantInferShell,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SemverError(#[from] semver::SemVerError),
}

pub struct Completions {
    pub shell: Option<Shell>,
    pub list: bool,
}

impl Command for Completions {
    type Error = FarmError;

    fn apply(&self, config: &FarmConfig) -> Result<(), Self::Error> {
        if self.list {
            for entry in config
                .versions_dir()
                .read_dir()
                .map_err(FarmError::IoError)?
            {
                let entry = entry.map_err(FarmError::IoError)?;
                if is_dotfile(&entry) {
                    continue;
                }

                let path = entry.path();
                let filename = path
                    .file_name()
                    .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
                    .map_err(FarmError::IoError)?
                    .to_str()
                    .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
                    .map_err(FarmError::IoError)?;
                let version = Version::parse(filename).map_err(FarmError::SemverError)?;
                outln!(config#Info, "{} {}", " ", version);
            }
            return Ok(());
        }

        let shell = self
            .shell
            .or_else(|| infer_shell().map(Into::into))
            .ok_or(FarmError::CantInferShell)?;

        print!(
            "{}",
            customize_completions(shell).expect("invalid completions")
        );
        Ok(())
    }
}

fn customize_completions(shell: Shell) -> Option<String> {
    use std::io::BufWriter;
    let mut buffer = BufWriter::new(Vec::new());
    build_cli().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut buffer);
    let bytes = buffer.into_inner().unwrap();
    let string = String::from_utf8(bytes).unwrap();
    let string_split = string.split('\n');
    let mut completions = String::new();
    let mut subcommand = FarmCommand::None;
    let use_command_regex =
        regex::Regex::new(format!(r#"(\s+){}{} "#, USE_COMMAND_REGEX, "<version>").as_str())
            .unwrap();
    let install_command_regex =
        regex::Regex::new(format!(r#"(\s+){}{} "#, INSTALL_COMMAND_REGEX, "<version>").as_str())
            .unwrap();
    let uninstall_command_regex =
        regex::Regex::new(format!(r#"(\s+){}{} "#, UNINSTALL_COMMAND_REGEX, "<version>").as_str())
            .unwrap();
    match shell {
        Shell::Zsh => {
            for (index, line) in string_split.clone().enumerate() {
                if index == string_split.clone().count() - 1 {
                    break;
                }
                subcommand = match line {
                    "(local)" => FarmCommand::Local,
                    "(global)" => FarmCommand::Global,
                    "(install)" => FarmCommand::Install,
                    "(uninstall)" => FarmCommand::Uninstall,
                    _ => subcommand,
                };
                completions.push_str(
                    format!(
                        "{}\n",
                        match subcommand {
                            FarmCommand::Local => match line {
                                r#"'::version:_files' \"# =>
                                    r#"'::version:_values 'version' $(farm completions --list)' \"#
                                        .to_string(),
                                _ => line.to_string(),
                            },
                            FarmCommand::Global => match line {
                                r#"':version:_files' \"# =>
                                    r#"':version:_values 'version' $(farm completions --list)' \"#
                                        .to_string(),
                                _ => line.to_string(),
                            },
                            FarmCommand::Install => match line {
                                r#"'::version:_files' \"# =>
                                    r#"'::version:_values 'version' $(farm install -l)' \"#
                                        .to_string(),
                                _ => line.to_string(),
                            },
                            FarmCommand::Uninstall => match line {
                                r#"':version:_files' \"# =>
                                    r#"':version:_values 'version' $(farm install -l)' \"#
                                        .to_string(),
                                _ => line.to_string(),
                            },
                            FarmCommand::None => line.to_string(),
                        }
                    )
                    .as_str(),
                );
            }
            Some(completions)
        }
        Shell::Bash => {
            for (index, line) in string_split.clone().enumerate() {
                if index == string_split.clone().count() - 1 {
                    break;
                }
                subcommand = if line.ends_with("farm__local)") {
                    FarmCommand::Local
                } else if line.ends_with("farm__global)") {
                    FarmCommand::Global
                } else if line.ends_with("farm__install)") {
                    FarmCommand::Install
                } else if line.ends_with("farm__uninstall)") {
                    FarmCommand::Uninstall
                } else {
                    subcommand
                };
                completions.push_str(
                    format!(
                        "{}\n",
                        match subcommand {
                            FarmCommand::Local =>
                                if use_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(farm completions --list) ""#,
                                        use_command_regex
                                            .captures(line)
                                            .unwrap()
                                            .get(1)
                                            .unwrap()
                                            .as_str(),
                                        USE_COMMAND_REGEX
                                    )
                                } else {
                                    line.to_string()
                                },
                            FarmCommand::Global =>
                                if use_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(farm completions --list) ""#,
                                        use_command_regex
                                            .captures(line)
                                            .unwrap()
                                            .get(1)
                                            .unwrap()
                                            .as_str(),
                                        USE_COMMAND_REGEX
                                    )
                                } else {
                                    line.to_string()
                                },
                            FarmCommand::Install =>
                                if install_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(farm install -l) ""#,
                                        install_command_regex
                                            .captures(line)
                                            .unwrap()
                                            .get(1)
                                            .unwrap()
                                            .as_str(),
                                        INSTALL_COMMAND_REGEX
                                    )
                                } else {
                                    line.to_string()
                                },
                            FarmCommand::Uninstall =>
                                if uninstall_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(farm install -l) ""#,
                                        uninstall_command_regex
                                            .captures(line)
                                            .unwrap()
                                            .get(1)
                                            .unwrap()
                                            .as_str(),
                                        UNINSTALL_COMMAND_REGEX
                                    )
                                } else {
                                    line.to_string()
                                },
                            FarmCommand::None => line.to_string(),
                        }
                    )
                    .as_str(),
                );
            }
            Some(completions)
        }
        _ => None,
    }
}

fn shells_as_string() -> String {
    AVAILABLE_SHELLS
        .iter()
        .map(|x| format!("* {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::customize_completions;
    use crate::config::FarmConfig;
    use clap::Shell;
    use difference::assert_diff;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use tempfile::tempdir;

    #[test]
    fn test_zsh_completions() {
        let mut config = FarmConfig::default();
        config.base_dir = Some(tempdir().unwrap().path().to_path_buf());

        let file = File::open("completions/farm.zsh").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut expected = String::new();
        buf_reader.read_to_string(&mut expected).unwrap();
        let actual = customize_completions(Shell::Zsh).unwrap();
        assert_diff!(actual.as_str(), expected.as_str(), "\n", 0);
    }

    #[test]
    fn test_bash_completions() {
        let mut config = FarmConfig::default();
        config.base_dir = Some(tempdir().unwrap().path().to_path_buf());

        let file = File::open("completions/farm.bash").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut expected = String::new();
        buf_reader.read_to_string(&mut expected).unwrap();
        let actual = customize_completions(Shell::Bash).unwrap();
        assert_diff!(actual.as_str(), expected.as_str(), "\n", 0);
    }
}