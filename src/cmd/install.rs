use crate::config::Config;
use crate::downloader::download;
use crate::fetcher::Fetcher;
use crate::version::{NodeVersion, Version};
use clap::Clap;
use colored::*;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Install {
    /// A version string. Can be a partial semver or a LTS version name by the format lts/NAME
    version: Version,
}

impl super::Command for Install {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let can_install = match &self.version {
            Version::Full(NodeVersion::Lts(_)) => false,
            Version::Full(NodeVersion::Alias(_)) => false,
            _ => true,
        };

        if !can_install {
            return Err(anyhow::Error::msg(format!(
                "Requested version ({}) is not installable",
                &self.version
            )));
        }

        let release = Fetcher::fetch(&config.dist_mirror)?.find_release(&self.version);

        match release {
            Some(r) => {
                download(&r, &config)?;
                Ok(())
            }
            _ => Err(anyhow::Error::msg(format!(
                "No release found with the version {}",
                &self.version.to_string().bold()
            ))),
        }
    }
}
