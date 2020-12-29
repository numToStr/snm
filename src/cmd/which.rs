use crate::config::Config;
use crate::version::{NodeVersion, Version};
use clap::Clap;
use std::path::PathBuf;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Which {
    /// Can be a partial semver string.
    version: Version,
}

impl super::Command for Which {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let dir = config.release_dir();
        let versions = NodeVersion::list_versions(&dir)?;
        let mut versions = self.version.match_node_versions(&versions).into_iter();

        if versions.len() == 1 {
            println!(
                "{}",
                pretty_path(dir.join(versions.next().unwrap().version_str()))
            )
        } else {
            for ver in versions {
                println!("- {}\t{}", ver, pretty_path(dir.join(ver.version_str())))
            }
        }

        Ok(())
    }
}

fn pretty_path(path: PathBuf) -> std::string::String {
    super::bin_path(path).join("node").display().to_string()
}
