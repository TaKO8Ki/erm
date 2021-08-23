use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("frum")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.1")
        .about("A little bit fast and modern Ruby version manager written in Rust")
        .arg(
            Arg::with_name("log-level")
                .long("log-level")
                .help("The log level of frum commands [default: info] [possible values: quiet, info, error]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ruby-build-mirror")
                .long("ruby-build-mirror")
                .help("[default: https://cache.ruby-lang.org/pub/ruby]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("base-dir")
                .long("frum-dir")
                .help("The root directory of frum installations [default: $HOME/.frum]")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("init").about("Sets environment variables for initializing frum"),
        )
        .subcommand(
            SubCommand::with_name("install")
                .setting(AppSettings::TrailingVarArg)
                .setting(AppSettings::AllowLeadingHyphen)
                .about("Installs a specific Ruby version")
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("Lists Ruby versions available to install"),
                )
                .arg(Arg::with_name("version").index(1))
                .arg(
                    Arg::with_name("configure_opts")
                        .multiple(true)
                        .help("Options passed to ./configure"),
                ),
        )
        .subcommand(
            SubCommand::with_name("uninstall")
                .about("Uninstall a specific Ruby version")
                .arg(Arg::with_name("version").index(1).required(true)),
        )
        .subcommand(SubCommand::with_name("versions").about("Lists installed Ruby versions"))
        .subcommand(
            SubCommand::with_name("local")
                .about("Sets the current Ruby version")
                .arg(Arg::with_name("version").index(1)),
        )
        .subcommand(
            SubCommand::with_name("global")
                .about("Sets the global Ruby version")
                .arg(Arg::with_name("version").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("completions")
                .about("Print shell completions to stdout")
                .arg(
                    Arg::with_name("shell")
                        .short("s")
                        .long("shell")
                        .help("The shell syntax to use")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("Lists installed Ruby versions")
                        .hidden(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("clean").about("Remove all downloads")
        )
}
