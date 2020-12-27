use crate::alias::Alias;
use crate::config::Config;
use crate::pretty_error;
use crate::version::NodeVersion;
use clap::Clap;
use colored::*;
use std::fs;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Prune;

impl super::Command for Prune {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let default_alias = config.alias_default();

        if !default_alias.exists() {
            return pretty_error!("Unable to prune. No {} alias found", "default".bold());
        }

        // Removing aliases except the `default` alias
        for alias in Alias::list(config.alias_dir())? {
            if alias.alias_path != default_alias {
                alias.remove_alias()?;
            }
        }

        // Removing all the versions except the one which is aliased to `default`
        let dest_path = fs::read_link(&default_alias)?;
        let alias = Alias::new(default_alias, dest_path);
        let dir = config.release_dir();
        for release in NodeVersion::list_versions(&dir)? {
            let release = release.version_str();
            let to_delete = dir.join(&release);
            if alias.version_str() != release && to_delete.exists() {
                fs::remove_dir_all(to_delete)?;
            }
        }

        Ok(())
    }
}
