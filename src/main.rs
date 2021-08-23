mod alias;
mod archive;
mod cli;
mod command;
mod commands;
mod config;
mod input_version;
mod remote_ruby_index;
mod shell;
mod symlink;
mod version;
mod version_file;

#[macro_use]
mod log;

use command::Command;
use std::str::FromStr;

fn main() {
    env_logger::init();
    let matches = cli::build_cli().get_matches();

    let mut config = config::FrumConfig::default();
    if let Some(log_level) = matches.value_of("log-level") {
        config.log_level = log::LogLevel::from_str(log_level).expect("invalid log level")
    }
    if let Some(ruby_build_mirror) = matches.value_of("ruby-build-mirror") {
        config.ruby_build_mirror =
            reqwest::Url::parse(ruby_build_mirror).expect("invalid ruby build mirror")
    };
    if let Some(base_dir) = matches.value_of("base-dir") {
        config.base_dir = Some(std::path::PathBuf::from(base_dir))
    };
    match matches.subcommand() {
        ("init", _) => commands::init::Init {}.call(&config),
        ("versions", _) => commands::versions::Versions {}.call(&config),
        ("global", Some(sub_matches)) => commands::global::Global {
            version: input_version::InputVersion::from_str(
                sub_matches.value_of("version").unwrap(),
            )
            .expect("invalid version"),
        }
        .call(&config),
        ("local", Some(sub_matches)) => commands::local::Local {
            version: sub_matches.value_of("version").map(|version| {
                input_version::InputVersion::from_str(version).expect("invalid version")
            }),
        }
        .call(&config),
        ("install", Some(sub_matches)) => {
            if sub_matches.is_present("list") {
                commands::install_list::InstallList {}.call(&config);
                return;
            }
            commands::install::Install {
                version: sub_matches.value_of("version").map(|version| {
                    input_version::InputVersion::from_str(version).expect("invalid version")
                }),
                configure_opts: match sub_matches.values_of("configure_opts") {
                    Some(opts) => opts.map(|opt| opt.to_string()).collect(),
                    None => Vec::new(),
                },
            }
            .call(&config);
        }
        ("uninstall", Some(sub_matches)) => {
            commands::uninstall::Uninstall {
                version: input_version::InputVersion::from_str(
                    sub_matches.value_of("version").unwrap(),
                )
                .expect("invalid version"),
            }
            .call(&config);
        }
        ("completions", Some(sub_matches)) => {
            commands::completions::Completions {
                shell: sub_matches
                    .value_of("shell")
                    .map(|shell| clap::Shell::from_str(shell).expect("invalid shell")),
                list: sub_matches.is_present("list"),
            }
            .call(&config);
        }
        ("clean", _) => commands::clean::Clean {}.call(&config),
        _ => (),
    };
}
