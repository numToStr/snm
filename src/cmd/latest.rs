use crate::fetcher::Fetcher;
use crate::{config::Config, downloader::Downloader};
use clap::Clap;
use colored::*;

const ALIAS: &str = "latest";

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Latest;

impl super::Command for Latest {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let releases = Fetcher::fetch(&config.dist_mirror)?;
        let release = releases.latest()?;

        let dwnld = Downloader::new(release, &config);
        let dest = dwnld.download()?;

        crate::symlink::symlink_to(&dest, &config.alias_dir().join(&ALIAS))?;

        println!("Alias     : {}", ALIAS.bold());

        if !config.no_use {
            dwnld.alias_to_default(&dest)?;
        }

        Ok(())
    }
}
