use crate::commands::{
    alias, completions, env, exec, install, latest, ls, ls_remote, lts, purge, r#use, unalias,
    uninstall, which, Command,
};
use clap::Subcommand;
use snm_core::SnmRes;

use super::Config;

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Alias a version to a common name
    #[clap(visible_alias = "a")]
    Alias(alias::Alias),

    /// Sets up the shell variables for snm
    Env(env::Env),

    /// Executes a command within snm context with the modified PATH
    ///
    /// Example: snm exec 14 -- node -v | snm exec 14 -- yarn start
    Exec(exec::Exec),

    /// Prints shell's completion script for snm to the stdout
    Completions(completions::Completions),

    /// Install Nodejs with the given version or lts codename
    ///
    /// Example: snm install 14 | snm install lts/fermium
    #[clap(visible_alias = "i")]
    Install(install::Install),

    /// Changes Nodejs version
    ///
    /// NOTE: If the <version> is not given, then version will be picked from .nvmrc or .node-version
    #[clap(visible_alias = "as")]
    Use(r#use::Use),

    /// Install the latest CURRENT release
    Latest(latest::Latest),

    /// Install the latest LTS release
    Lts(lts::Lts),

    /// List all the local installed versions with their aliases
    Ls(ls::Ls),

    /// List remote Nodejs versions
    #[clap(visible_alias = "lsr")]
    LsRemote(ls_remote::LsRemote),

    /// Output path for installed node version
    Which(which::Which),

    /// Remove everything except the active version.
    ///
    /// NOTE: This will also remove any redundant downloads
    #[clap(visible_alias = "prune")]
    Purge(purge::Purge),

    /// Unlink the alias
    ///
    /// NOTE: This only removes the alias and doesn't remove the linked version
    #[clap(name = "unalias", visible_alias = "rma")]
    UnAlias(unalias::UnAlias),

    /// Remove the installed version or alias
    ///
    /// Example: snm uninstall 14 | snm uninstall lts-fermium
    ///
    /// NOTE: If given an alias like ten or lts-fermium then it will remove the version that the alias is pointing to and all the aliases which are pointing to the same version.
    /// Also, If multiple installation were found for a version, then it will remove the latest.
    #[clap(name = "uninstall", visible_alias = "rm")]
    UnInstall(uninstall::UnInstall),
}

impl SubCommand {
    pub fn exec(self, config: Config) -> SnmRes<()> {
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
            Self::Purge(m) => m.init(config),
            Self::UnAlias(m) => m.init(config),
            Self::UnInstall(m) => m.init(config),
        }
    }
}
