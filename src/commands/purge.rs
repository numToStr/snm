use crate::cli::Config;
use clap::Clap;
use console::style;
use snm_core::{linker::Linker, types::UserAlias, version::DistVersion, SnmRes};
use std::fs::remove_dir_all;

#[derive(Debug, Clap)]
pub struct Purge {
    /// Remove everything, including the active version
    #[clap(short, long)]
    all: bool,
}

impl super::Command for Purge {
    fn init(self, config: Config) -> SnmRes<()> {
        // If all=true then nuke the snm home directory
        if self.all {
            let snm_home = config.snm_home();

            if snm_home.exists() {
                remove_dir_all(snm_home)?;
            }

            println!("Purge complete!");

            return Ok(());
        }

        let default_alias = config.alias_default();

        if !default_alias.as_ref().exists() {
            anyhow::bail!(
                "Unable to prune. No {} alias found",
                style(UserAlias::DEFAULT).bold()
            );
        }

        let release_dir = config.release_dir();

        let used_ver = Linker::read_convert_to_dist(&default_alias, &release_dir)?;

        // Nuke the alias directory after reading the default alias
        remove_dir_all(config.alias_dir().as_ref())?;

        // Nuke the download directory, to cleanup any redundant downloads
        remove_dir_all(config.download_dir().as_ref())?;

        // Removing all the versions except the one which is aliased to `default`
        let dist_versions = DistVersion::list_versions(&release_dir)?;
        for version in dist_versions {
            // If the version is currently active then don't delete
            if version.eq(&used_ver) {
                continue;
            }

            let to_delete = release_dir.join(version.to_string());

            if to_delete.as_ref().exists() {
                remove_dir_all(to_delete.as_ref())?;
            }
        }

        // Restoring the default alias
        // NOTE: don't use the `default_alias` variable
        Linker::create_link(
            &release_dir.join(used_ver.to_string()),
            &config.alias_default(),
        )?;

        println!("Purge complete!");

        Ok(())
    }
}
