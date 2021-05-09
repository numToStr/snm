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
            Version::Full(NodeVersion::Alias(_)) => false,
            _ => true,
        };

        if !can_install {
            anyhow::bail!(
                "Unable to install the version {}",
                &self.version.to_string().bold()
            );
        }

        let (release, is_lts) = match &self.version {
            Version::Full(NodeVersion::Lts(lts)) => {
                (Fetcher::fetch(&config.dist_mirror)?.lts_name(lts), true)
            }
            _ => (
                Fetcher::fetch(&config.dist_mirror)?.find_release(&self.version),
                false,
            ),
        };

        match release {
            Some(r) => {
                let dest = download(&r, &config)?;
                if is_lts {
                    let alias = self.version.to_string();
                    crate::symlink::symlink_to(&dest, &config.alias_dir().join(&alias))?;
                    println!("Alias     : {}", alias.bold());
                }
                Ok(())
            }
            _ => anyhow::bail!(
                "No release found with the version {}",
                &self.version.to_string().bold()
            ),
        }
    }
}
