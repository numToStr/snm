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

    /// Executes a command within snm context with the modified PATH
    ///
    /// Example: snm exec 14 -- node -v | snm exec 14 -- yarn start
    #[clap(name = "exec")]
    Exec(exec::Exec),

    /// Prints shell's completion script for snm to the stdout
    #[clap(name = "completions")]
    Completions(completions::Completions),

    /// Install Nodejs with the given version or lts codename
    ///
    /// Example: snm install 14 | snm install lts/fermium
    #[clap(name = "install", visible_alias = "i")]
    Install(install::Install),

    /// Changes Nodejs version
    ///
    /// NOTE: If the <version> is not given, then version will be picked from .nvmrc or .node-version
    #[clap(name = "use", visible_alias = "as")]
    Use(r#use::Use),

    /// Install the latest CURRENT release
    #[clap(name = "latest")]
    Latest(latest::Latest),

    /// Install the latest LTS release
    #[clap(name = "lts")]
    Lts(lts::Lts),

    /// List all the local downloaded versions with their aliases (if any)
    #[clap(name = "ls")]
    Ls(ls::Ls),

    /// List remote Nodejs versions
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

    /// Remove the installed Nodejs with the given version or alias
    ///
    /// Example: snm uninstall 14 | snm uninstall lts-fermium
    ///
    /// NOTE: If given an alias like ten or lts-fermium then it will remove the version which the alias is pointing at and all the aliases which are pointing to the same version.
    /// Also, uninstalling a version will throw an error, if multiple installation is found in the same semver range
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

const VERSION_HELP: &'static str = r#"
Versions:

    Numeric version numbers can be complete or partial semver, with an optional leading 'v'.
    Versions can also be specified by their codename, prefixed with lts- or lts/.

    4.9.1, 8, v6.1              Numeric versions
    lts/boron, lts/carbon       Codenames for release streams
"#;

#[derive(Clap, Debug)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    after_help = VERSION_HELP
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
