use crate::config::Config;
use crate::directory::symlink_to;
use crate::version::{NodeVersion, Version};
use clap::Clap;
use std::str::FromStr;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Use {
    /// A version string. Can be a partial semver or a LTS version name by the format lts/NAME
    version: String,
}

impl super::Command for Use {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let parsed = Version::from_str(&self.version)?;
        let dir = config.release_dir();
        let downloads = NodeVersion::list_versions(&dir)?;

        let version = parsed
            .to_node_version(&downloads)
            .ok_or(anyhow::Error::msg("Unable to find the version."))?;

        symlink_to(dir.join(version.version_str()), config.alias_default())?;

        println!("Using Node.js {}", version);

        Ok(())
    }
}
