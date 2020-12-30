use crate::config::Config;
use clap::Clap;
use colored::*;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct UnAlias {
    /// Name of the alias
    #[clap(conflicts_with = "all")]
    alias: Option<String>,

    /// Remove all the aliases
    #[clap(short, long)]
    all: bool,
}

impl super::Command for UnAlias {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let dir = config.alias_dir();

        if self.all {
            std::fs::remove_dir_all(dir)?;
            println!("Removed all the aliases");
            return Ok(());
        }

        let alias = crate::alias::sanitize(&self.alias.clone().unwrap());
        let path = dir.join(&alias);

        if !path.exists() {
            return crate::pretty_error!("Alias {} not found", &alias.bold());
        }

        crate::symlink::remove_symlink(path)?;

        println!("Removed alias: {}", &alias.bold());

        Ok(())
    }
}
