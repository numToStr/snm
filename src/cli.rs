use clap::Clap;

#[derive(Clap, Debug, PartialEq, Eq)]
pub enum SubCommand {
    /// Display downloaded node versions and install selection
    #[clap(name = "select")]
    Select,

    /// Install the latest node release (downloading if necessary)
    #[clap(name = "latest")]
    Latest,

    /// Install the latest LTS node release (downloading if necessary)
    #[clap(name = "lts")]
    Lts,

    /// Output downloaded versions
    #[clap(name = "ls")]
    Ls,

    /// Output downloaded versions
    #[clap(name = "ls-remote", aliases = &["lsr"])]
    LsRemote,

    /// Remove all downloaded versions except the installed version
    #[clap(name = "prune")]
    Prune,
}

#[derive(Clap, Debug)]
#[clap(
    version = "0.0.1",
    author = "Vikas Raj <vikasraj11@gmail.com>",
    about = "Simple and Stupid Node Manager"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

impl Cli {
    pub fn new() -> Self {
        return Self::parse();
    }
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
