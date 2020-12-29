use crate::cmd::{
    alias, completions, env, exec, install, latest, ls, ls_remote, lts, prune, r#use, unalias,
    uninstall, which, Command,
};
use crate::config::Config;
use clap::{crate_authors, crate_description, crate_name, crate_version, Clap};

#[derive(Clap, Debug, PartialEq, Eq)]
pub enum SubCommand {
    /// Alias a version to a common name
    #[clap(name = "alias")]
    Alias(alias::Alias),

    /// Sets up the shell variables for snm
    #[clap(name = "env")]
    Env(env::Env),

    /// Executes a command within snm context w/ modified $PATH
    ///
    /// Example: snm exec 14 -- node -v
    #[clap(name = "exec")]
    Exec(exec::Exec),

    /// Prints shell's completion script for snm to the stdout
    #[clap(name = "completions")]
    Completions(completions::Completions),

    /// Install node <version> (downloading if necessary)
    #[clap(name = "install", visible_alias = "i")]
    Install(install::Install),

    /// Display downloaded node versions and install selection
    #[clap(name = "use", visible_alias = "as")]
    Use(r#use::Use),

    /// Install the latest node release (downloading if necessary)
    #[clap(name = "latest")]
    Latest(latest::Latest),

    /// Install the latest LTS node release (downloading if necessary)
    #[clap(name = "lts")]
    Lts(lts::Lts),

    /// List all the local downloaded versions w/ alias (if any)
    #[clap(name = "ls")]
    Ls(ls::Ls),

    /// List remote Node.js versions
    #[clap(name = "ls-remote", visible_alias = "lsr")]
    LsRemote(ls_remote::LsRemote),

    /// Output path for downloaded node <version>
    #[clap(name = "which")]
    Which(which::Which),

    /// Remove all downloaded versions except the installed version
    #[clap(name = "prune")]
    Prune(prune::Prune),

    /// Remove the aliases
    #[clap(name = "unalias", visible_alias = "rma")]
    UnAlias(unalias::UnAlias),

    /// Remove the installed node and npm
    #[clap(name = "uninstall", visible_alias = "rm")]
    UnInstall(uninstall::UnInstall),
}

impl SubCommand {
    pub fn exec(&self, config: Config) -> anyhow::Result<()> {
        match self {
            Self::Alias(m) => m.init(config),
            Self::Env(m) => m.init(config),
            Self::Exec(m) => m.init(config),
            Self::Completions(m) => m.init(config),
            Self::Install(m) => m.init(config),
            Self::Use(m) => m.init(config),
            Self::Latest(m) => m.init(config),
            Self::Lts(m) => m.init(config),
            Self::Ls(m) => m.init(config),
            Self::LsRemote(m) => m.init(config),
            Self::Which(m) => m.init(config),
            Self::Prune(m) => m.init(config),
            Self::UnAlias(m) => m.init(config),
            Self::UnInstall(m) => m.init(config),
        }
    }
}

#[derive(Clap, Debug)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!()
)]
pub struct Cli {
    #[clap(flatten)]
    pub options: Config,

    #[clap(subcommand)]
    pub cmd: SubCommand,
}

pub fn parse() -> Cli {
    return Cli::parse();
}

// n use                          Display downloaded node versions and install selection
// n latest                       Install the latest node release (downloading if necessary)
// n lts                          Install the latest LTS node release (downloading if necessary)
// n install <version>            Install node <version> (downloading if necessary)
// n prune                        Remove all downloaded versions except the installed version
// n ls                           Output downloaded versions
// n ls-remote [version]          Output matching versions available for download
// n uninstall                    Remove the installed node and npm
//
// n run <version> [args ...]     Execute downloaded node <version> with [args ...]
// n which <version>              Output path for downloaded node <version>
// n exec <vers> <cmd> [args...]  Execute command with modified PATH, so downloaded node <version> and npm first
// n --latest                     Output the latest node version available
// n --lts                        Output the latest LTS node version available
//
// n rm <version ...>             Remove the given downloaded version(s)
