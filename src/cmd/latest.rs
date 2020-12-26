use crate::config::Config;
use crate::downloader::Downloader;
use crate::fetcher::Fetcher;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Latest;

impl super::Command for Latest {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let mut release = Fetcher::fetch(&config.dist_mirror)?;
        let release = release.latest()?;

        Downloader.download(release, &config)?;

        Ok(())
    }
}
