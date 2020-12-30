use crate::config::Config;
use crate::symlink::symlink_to;
use crate::version::{NodeVersion, Version};
use clap::Clap;
use colored::*;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Use {
    /// Can be a partial semver or a LTS version name by the format lts/NAME
    version: Version,
}

impl super::Command for Use {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        match &self.version {
            // Get the link dest of the alias and symlink default to dest rather than symlinking to alias
            // both Alias(_) and Lts() are treated as symlinks
            Version::Full(NodeVersion::Lts(_)) | Version::Full(NodeVersion::Alias(_)) => {
                let alias = crate::alias::sanitize(&self.version.to_string());
                let link = config.alias_dir().join(&alias);

                if link.exists() {
                    let dest = std::fs::read_link(link)?;

                    symlink_to(&dest, &config.alias_default())?;

                    println!("Using Alias {}", &alias.bold());
                }
            }
            _ => {
                let dir = config.release_dir();
                let versions = NodeVersion::list_versions(&dir)?;

                let version = self.version.to_node_version(&versions)?;

                symlink_to(dir.join(version.version_str()), config.alias_default())?;
                println!("Using Node.js {}", version.version_str().bold());
            }
        };

        Ok(())
    }
}
