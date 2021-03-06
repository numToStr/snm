use crate::alias::{self, Alias};
use crate::config::Config;
use crate::version::{NodeVersion, Version};
use clap::Clap;
use colored::*;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct UnInstall {
    /// Version that needs to be removed. Can be a partial semver string.
    version: Version,

    /// Don't remove if the version is currently used.
    #[clap(short, long)]
    no_used: bool,
}

impl super::Command for UnInstall {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let dir = config.release_dir();
        let found_ver = match self.version {
            // If given version is an alias or lts/* then read the link
            // and return the parsed NodeVersion
            Version::Full(NodeVersion::Alias(_)) | Version::Full(NodeVersion::Lts(_)) => {
                let alias = crate::alias::sanitize(&self.version.to_string());
                let link = config.alias_dir().join(&alias);

                if !link.exists() {
                    anyhow::bail!("Alias {} not found", &alias.bold());
                }

                let aliased = Alias::new(link);
                let dest = aliased.destination()?;

                aliased.remove()?;

                println!("Removed alias: {}", alias.bold());

                let version = alias::pretty_path_name(&dest);

                Some(NodeVersion::parse(version)?)
            }
            _ => {
                let downloaded = NodeVersion::list_versions(&dir)?;
                let matches = self.version.match_node_versions(&downloaded);

                if matches.is_empty() {
                    anyhow::bail!(
                        "No downloads found with version {}",
                        &self.version.to_string().bold()
                    );
                }

                if matches.len() > 1 {
                    eprintln!(
                        "Multiple versions found, expected 1. Please be a little more specific."
                    );
                    for m in matches {
                        eprintln!("- {}", m);
                    }
                    None
                } else {
                    Some(matches.into_iter().next().unwrap().clone())
                }
            }
        };

        if let Some(ver) = found_ver {
            let aliases = Alias::list_for_version(config.alias_dir(), &ver)?;

            for alias in aliases {
                if alias.name() == "default" && self.no_used {
                    anyhow::bail!(
                        "Unable to uninstall. Version {} is currently used",
                        ver.to_string().bold()
                    );
                }

                alias.remove()?;
                println!("Removed alias: {}", alias.name().bold());
            }

            std::fs::remove_dir_all(dir.join(ver.version_str()))?;
            println!("Removed version: {}", ver.to_string().bold());
        }

        Ok(())
    }
}
