use crate::config::Config;
use crate::downloader::Downloader;
use crate::fetcher::Releases;
use crate::version::Version;
use clap::Clap;
use std::str::FromStr;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Install {
    version: String,
}

impl super::Command for Install {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let ver = Version::from_str(&self.version)?;
        let release = Releases::fetch()?.find_release(&ver);

        match release {
            Some(r) => {
                Downloader.download(&r, &config)?;
            }
            _ => println!("No release found with the version {}", ver),
        }

        Ok(())
    }
}
