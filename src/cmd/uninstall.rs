use crate::alias::Alias;
use crate::config::Config;
use crate::version::{NodeVersion, Version};
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct UnInstall {
    /// A version string. Can be a partial semver or a LTS version name by the format lts/NAME
    version: Version,

    /// Don't remove if the version is currently used.
    #[clap(short, long)]
    no_used: bool,
}

impl super::Command for UnInstall {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let dir = config.release_dir();
        let downloaded = NodeVersion::list_versions(&dir)?;
        let matches: Vec<&NodeVersion> = downloaded
            .iter()
            .filter(|&d| self.version.match_version(d))
            .collect();

        if matches.is_empty() {
            println!("No downloaded version found.");
            return Ok(());
        }

        if matches.len() > 1 {
            println!("Multiple versions found, expected 1. Please be a little more specific.");
            for m in matches {
                println!("- {}", m);
            }
        } else {
            let found_ver = matches.get(0).unwrap();
            let aliases = Alias::list(config.alias_dir())?;
            let found_alias = found_ver.list_aliases(&aliases);

            for alias in found_alias {
                if alias.name() == "default" && self.no_used {
                    println!("{} is currently used. Aborting...", found_ver);
                    return Ok(());
                }

                alias.remove_alias()?;
                println!("Removed alias: {}", alias.name());
            }

            std::fs::remove_dir_all(dir.join(found_ver.version_str()))?;
            println!("Removed version: {}", found_ver);
        }

        Ok(())
    }
}
