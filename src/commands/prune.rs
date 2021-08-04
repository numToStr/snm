use crate::config::Config;
use clap::Clap;
use console::style;
use snm_core::{linker::Linker, version::dist_version::DistVersion, SnmRes};
use std::fs::remove_dir_all;

#[derive(Debug, Clap)]
pub struct Prune;

impl super::Command for Prune {
    fn init(self, config: Config) -> SnmRes<()> {
        let default_alias = config.alias_default();

        if !default_alias.exists() {
            anyhow::bail!(
                "Unable to prune. No {} alias found",
                style("default").bold()
            );
        }

        let release_dir = config.release_dir();

        let used_ver = Linker::read_convert_to_dist(&default_alias, &release_dir)?;

        // Nuke the alias directory after reading the default alias
        remove_dir_all(config.alias_dir())?;

        // Nuke the download directory, to cleanup any redundant downloads
        remove_dir_all(&config.download_dir())?;

        // Removing all the versions except the one which is aliased to `default`
        let dist_versions = DistVersion::list_versions(&release_dir)?;
        for version in dist_versions {
            // If the version is currently used then don't deleted
            if version.eq(&used_ver) {
                continue;
            }

            let to_delete = release_dir.join(version.to_string());

            if to_delete.exists() {
                remove_dir_all(to_delete)?;
            }
        }

        // Restoring the default alias
        // NOTE: don't use the `default_alias` variable
        Linker::create_link(
            &release_dir.join(used_ver.to_string()),
            &config.alias_default(),
        )?;

        Ok(())
    }
}
