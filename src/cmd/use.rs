use crate::alias::Alias;
use crate::config::Config;
use crate::lib::SnmRes;
use crate::symlink::symlink_to;
use crate::version::{NodeVersion, Version};
use clap::Clap;
use colored::*;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Use {
    /// Can be a partial semver or a LTS version name by the format lts/NAME.
    version: Option<Version>,
}

impl super::Command for Use {
    fn init(self, config: Config) -> SnmRes<()> {
        // If version is not provided then fetch version from file
        let version = match &self.version {
            Some(v) => v.clone(),
            None => Version::from_file()?.ok_or_else(|| {
                anyhow::anyhow!(
                    "Unable to read version from dotfiles. Please provide a version manually."
                )
            })?,
        };

        match version {
            // Get the link dest of the alias and symlink default to dest rather than symlinking to alias
            // both Alias(_) and Lts() are treated as symlinks
            Version::Full(NodeVersion::Lts(_)) | Version::Full(NodeVersion::Alias(_)) => {
                let alias = crate::alias::sanitize(&version.to_string());
                let link = config.alias_dir().join(&alias);

                if !link.exists() {
                    anyhow::bail!("Alias {} not found", &alias.bold());
                }

                let dest = Alias::new(link).destination()?;

                symlink_to(&dest, &config.alias_default())?;

                println!("Using Alias {}", &alias.bold());
            }
            _ => {
                let dir = config.release_dir();
                let versions = NodeVersion::list_versions(&dir)?;

                let version = version.to_node_version(&versions)?;

                symlink_to(dir.join(version.version_str()), config.alias_default())?;
                println!("Using Node.js {}", version.version_str().bold());
            }
        };

        Ok(())
    }
}
