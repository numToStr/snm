use crate::alias::Alias;
use crate::config::Config;
use crate::pretty_error;
use crate::version::{NodeVersion, Version};
use clap::Clap;
use colored::*;

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
        let matches = self.version.match_node_versions(&downloaded);

        if matches.is_empty() {
            return pretty_error!(
                "No downloads found with version {}",
                &self.version.to_string().bold()
            );
        }

        if matches.len() > 1 {
            eprintln!("Multiple versions found, expected 1. Please be a little more specific.");
            for m in matches {
                eprintln!("- {}", m);
            }
        } else {
            let found_ver = matches.get(0).unwrap();
            let aliases = Alias::list(config.alias_dir())?;
            let found_alias = found_ver.list_aliases(&aliases);

            for alias in found_alias {
                if alias.name() == "default" && self.no_used {
                    return Err(anyhow::Error::msg(format!(
                        "Unable to uninstall. Version {} is currently used",
                        found_ver.to_string().bold()
                    )));
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
