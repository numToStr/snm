use crate::config::Config;
use crate::symlink::symlink_to;
use crate::version::{NodeVersion, Version};
use clap::Clap;
use colored::*;

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
        let version = self.version.to_node_version(&versions)?;

        let alias = crate::alias::sanitize(&self.alias);

        symlink_to(
            dir.join(version.version_str()),
            config.alias_dir().join(&alias),
        )?;

        println!(
            "Version {} is aliased to {}",
            version.to_string().bold(),
            &alias.bold()
        );

        Ok(())
    }
}
