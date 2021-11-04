use crate::cli::Config;
use clap::Parser;
use console::style;
use snm_core::{linker::Linker, types::UserAlias, SnmRes};

#[derive(Debug, Parser)]
pub struct UnAlias {
    /// Name of the alias
    #[clap(conflicts_with = "all", required_unless_present = "all")]
    alias: Option<UserAlias>,

    /// Remove all the aliases
    #[clap(short, long)]
    all: bool,
}

impl super::Command for UnAlias {
    fn init(self, config: Config) -> SnmRes<()> {
        let alias_dir = config.alias_dir();

        if self.all {
            std::fs::remove_dir_all(alias_dir.as_ref())?;
            println!("Removed all the aliases");
            return Ok(());
        }

        if let Some(alias) = &self.alias {
            let alias_path = alias_dir.join(alias.as_ref());

            if !alias_path.as_ref().exists() {
                anyhow::bail!("Alias {} not found", style(alias).bold());
            }

            Linker::remove_link(&alias_path)?;

            println!("Removed alias: {}", style(alias).bold());
        };

        Ok(())
    }
}
