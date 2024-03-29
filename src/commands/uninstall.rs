use crate::cli::Config;
use clap::Parser;
use console::style;
use snm_core::{
    linker::Linker,
    types::UserAlias,
    version::{DistVersion, UserVersion},
    SnmRes,
};

#[derive(Debug, Parser)]
pub struct UnInstall {
    /// Semver, Alias or Lts codename that needs to be removed
    version: UserVersion,

    /// Forcefully remove the active version
    #[clap(short, long)]
    force: bool,
}

impl super::Command for UnInstall {
    fn init(self, config: Config) -> SnmRes<()> {
        let release_dir = config.release_dir();
        let alias_dir = config.alias_dir();

        // first we need to find out the whether the provided version is an alias, lts codename or partial semver
        // If the version is alias or codename, then we need to find the linked/installed version
        let version = match &self.version {
            UserVersion::Lts(lts_code) => {
                let alias_ver = alias_dir.join(&lts_code.to_string());

                if !alias_ver.as_ref().exists() {
                    anyhow::bail!("Codename {} not found", style(lts_code).bold());
                }

                Linker::read_convert_to_dist(&alias_ver, &release_dir)?
            }
            UserVersion::Alias(alias) => {
                let alias_ver = alias_dir.join(alias.as_ref());

                if !alias_ver.as_ref().exists() {
                    anyhow::bail!("Alias {} not found", style(alias).bold());
                }

                Linker::read_convert_to_dist(&alias_ver, &release_dir)?
            }
            x => DistVersion::match_version(&release_dir, x)?,
        };

        // So, when the linked version is found then we need to find the other linked aliases,
        // then remove them all the aliases before removing the actuall installed version
        let aliases = Linker::list_for_version(&version, &alias_dir, &release_dir)?;

        // Checking whether the version is currently active or not
        let is_default = aliases.iter().any(|x| *x == UserAlias::ACTIVE);

        if is_default && !self.force {
            anyhow::bail!(
                "Unable to uninstall! Version {} is currently active. Add --force to override this behavior",
                style(version).bold()
            );
        }

        let is_aliases_empty = aliases.is_empty();

        // Removing symlink first
        if !is_aliases_empty {
            for alias in &aliases {
                let alias = alias_dir.join(&alias);
                Linker::remove_link(&alias)?;
            }
        }

        // Then removing the actual installed version
        std::fs::remove_dir_all(release_dir.join(version.to_string()).as_ref())?;

        println!("Removed version: {}", style(version).bold());

        if !is_aliases_empty {
            println!("Removed aliases: {}", aliases.join(", "));
        }

        Ok(())
    }
}
