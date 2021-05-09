use crate::config::Config;
use crate::downloader::download;
use crate::fetcher::Fetcher;
use clap::Clap;
use colored::*;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Latest;

impl super::Command for Latest {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let releases = Fetcher::fetch(&config.dist_mirror)?;
        let release = releases.latest()?;

        let dest = download(&release, &config)?;

        let alias = "latest";
        crate::symlink::symlink_to(&dest, &config.alias_dir().join(&alias))?;
        println!("Alias     : {}", alias.bold());

        Ok(())
    }
}
