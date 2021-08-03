use crate::alias::{self, Alias};
use crate::config::Config;
use crate::lib::SnmRes;
use crate::version::NodeVersion;
use clap::Clap;
use colored::*;
use std::fs;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Prune;

impl super::Command for Prune {
    fn init(self, config: Config) -> SnmRes<()> {
        let default_alias = config.alias_default();

        if !default_alias.exists() {
            anyhow::bail!("Unable to prune. No {} alias found", "default".bold());
        }

        // Removing aliases except the `default` alias
        for alias in Alias::list(config.alias_dir())? {
            if alias.path != default_alias {
                alias.remove()?;
            }
        }

        // Removing all the versions except the one which is aliased to `default`
        let alias = Alias::new(default_alias);
        let dir = config.release_dir();
        for release in NodeVersion::list_versions(&dir)? {
            let release = release.version_str();
            let to_delete = dir.join(&release);
            if alias::pretty_path_name(&alias.destination()?) != release && to_delete.exists() {
                fs::remove_dir_all(to_delete)?;
            }
        }

        Ok(())
    }
}
