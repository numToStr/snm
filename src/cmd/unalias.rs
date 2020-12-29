use crate::config::Config;
use clap::Clap;
use colored::*;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct UnAlias {
    /// Name of the alias
    #[clap(conflicts_with = "all")]
    name: Option<String>,

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
        } else {
            let name = self.name.clone().unwrap();
            crate::symlink::remove_symlink(dir.join(&name))?;
            println!("Removed alias: {}", &name.bold());
        }

        Ok(())
    }
}
