use clap::{App, ArgMatches, SubCommand};

pub fn app() -> ArgMatches<'static> {
    App::new("snm")
        .version("0.0.1")
        .author("Vikas Raj <vikasraj11@gmail.com>")
        .about("Simple Node Manager")
        // .arg(
        //     Arg::with_name("config")
        //         .short("c")
        //         .long("config")
        //         .value_name("FILE")
        //         .help("Sets a custom config file")
        //         .takes_value(true),
        // )
        // .arg(
        //     Arg::with_name("INPUT")
        //         .help("Sets the input file to use")
        //         .required(true)
        //         .index(1),
        // )
        .subcommand(
            SubCommand::with_name("select")
                .about("Display downloaded node versions and install selection"),
        )
        .subcommand(
            SubCommand::with_name("latest")
                .about("Install the latest node release (downloading if necessary)"),
        )
        .subcommand(
            SubCommand::with_name("lts")
                .about("Install the latest LTS node release (downloading if necessary)"),
        )
        // .arg(
        //     Arg::with_name("version")
        //         .short("v")
        //         .help("Install node <version> (downloading if necessary)"),
        // )
        .get_matches()
}

// n                              Display downloaded node versions and install selection
// n latest                       Install the latest node release (downloading if necessary)
// n lts                          Install the latest LTS node release (downloading if necessary)
// n <version>                    Install node <version> (downloading if necessary)
// n run <version> [args ...]     Execute downloaded node <version> with [args ...]
// n which <version>              Output path for downloaded node <version>
// n exec <vers> <cmd> [args...]  Execute command with modified PATH, so downloaded node <version> and npm first
// n rm <version ...>             Remove the given downloaded version(s)
// n prune                        Remove all downloaded versions except the installed version
// n --latest                     Output the latest node version available
// n --lts                        Output the latest LTS node version available
// n ls                           Output downloaded versions
// n ls-remote [version]          Output matching versions available for download
// n uninstall                    Remove the installed node and npm
