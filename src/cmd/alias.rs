use crate::config::Config;
use crate::directory::symlink_to;
use crate::version::{NodeVersion, Version};
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Alias {
    /// Partial semver string
    version: Version,

    /// A string consist of alphanumeric digits
    alias: String,
}

impl super::Command for Alias {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let dir = config.release_dir();
        let versions = NodeVersion::list_versions(&dir)?;
        let found = self.version.to_node_version(&versions);

        if let Some(version) = found {
            symlink_to(
                dir.join(version.version_str()),
                config.alias_dir().join(&self.alias),
            )?;
            println!("Version {} is aliased to {}", version, &self.alias);
        } else {
            println!("Version {} not found locally", self.version);
        }

        Ok(())
    }
}
