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
        let dir = config.release_dir();
        let versions = NodeVersion::list_versions(&dir)?;

        let version = self.version.to_node_version(&versions)?;

        symlink_to(dir.join(version.version_str()), config.alias_default())?;

        println!("Using Node.js {}", version.version_str().bold());

        Ok(())
    }
}
