use crate::config::Config;
use crate::downloader::Downloader;
use crate::fetcher::Releases;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Latest;

impl super::Command for Latest {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let mut release = Releases::fetch()?;
        let release = release.latest()?;

        Downloader.download(release, &config)?;

        Ok(())
    }
}
