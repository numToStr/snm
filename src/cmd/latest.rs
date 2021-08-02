use crate::config::Config;
use crate::lib::{downloader2::Downloader2, fetcher2::Fetcher2, linker::Linker, SnmRes};

use clap::Clap;

const ALIAS: &str = "latest";

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Latest;

impl super::Command for Latest {
    type InitResult = ();

    fn init(&self, config: Config) -> SnmRes<Self::InitResult> {
        let releases = Fetcher2::fetch(&config.dist_mirror)?;
        let release = releases.latest()?;

        let dwnld = Downloader2::new(&config.dist_mirror, &release.version);
        let dest = dwnld.download(&config.release_dir())?;

        let linker = Linker::new(&dest);

        linker.create_link(&config.alias_dir().join(&ALIAS))?;

        println!("Alias     : {}", ALIAS);

        if !config.no_use {
            linker.create_link(&config.alias_default())?;
        }

        Ok(())
    }
}
