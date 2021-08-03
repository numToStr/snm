use crate::config::Config;
use crate::lib::{
    linker::Linker,
    version::{dist_version::DistVersion, user_version::UserVersion, ParseVersion},
    SnmRes,
};
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct UnInstall {
    /// Version or Alias that needs to be removed
    version_or_alias: UserVersion,

    /// Don't remove if the version is currently used.
    #[clap(short = 'N', long)]
    no_used: bool,
}

impl super::Command for UnInstall {
    type InitResult = ();

    fn init(&self, config: Config) -> SnmRes<Self::InitResult> {
        let release_dir = config.release_dir();
        let alias_dir = config.alias_dir();

        // first we need to find out the whether the provided version is an alias, lts codename or partial semver
        // If the version is alias or codename, then we need to find the linked/installed version
        let found_version = match &self.version_or_alias {
            UserVersion::Lts(l) | UserVersion::Alias(l) => {
                let alias_ver = alias_dir.join(l);

                if !alias_ver.exists() {
                    anyhow::bail!("Alias {} not found", l);
                }

                let linked = Linker::read_link(&alias_ver)?;

                let link_ver = linked.strip_prefix(&release_dir)?;

                match link_ver.to_str() {
                    Some(p) => {
                        let parsed = DistVersion::parse(p)?;
                        Some(parsed)
                    }
                    _ => None,
                }
            }
            x => {
                let dist_version = DistVersion::match_version(&release_dir, x)?;

                Some(dist_version)
            }
        };

        // So, when the linked version is found then we need to find the other linked aliases,
        // then remove them all the aliases before removing the actuall installed version
        if let Some(version) = found_version {
            let aliases = Linker::list_for_version(&version, &alias_dir, &release_dir)?;

            let is_default = aliases.iter().any(|x| x.as_str() == "default");

            if is_default && self.no_used {
                anyhow::bail!("Unable to uninstall. Version {} is currently used", version);
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
            std::fs::remove_dir_all(release_dir.join(version.to_string()))?;

            println!("Removed version: {}", version);

            if !is_aliases_empty {
                println!("Removed aliases: {}", aliases.join(", "));
            }
        }

        Ok(())
    }
}
