use crate::config::Config;
use crate::downloader::Downloader;
use crate::fetcher::Fetcher;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Lts;

impl super::Command for Lts {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let releases = Fetcher::fetch(&config.dist_mirror)?;
        let release = releases.lts()?;

        Downloader.download(&release, &config)?;

        Ok(())
    }
}
