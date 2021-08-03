use crate::config::Config;
use crate::lib::{downloader2::Downloader2, fetcher2::Fetcher2, linker::Linker, SnmRes};
use clap::Clap;

const ALIAS: &str = "lts-latest";

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Lts;

impl super::Command for Lts {
    fn init(self, config: Config) -> SnmRes<()> {
        let releases = Fetcher2::fetch(&config.dist_mirror)?;
        let release = releases.lts()?;

        let dwnld = Downloader2::new(&config.dist_mirror, &release.version);
        let dest = dwnld.download(&config.release_dir())?;

        Linker::create_link(&dest, &config.alias_dir().join(&ALIAS))?;

        println!("Alias     : {}", ALIAS);

        if !config.no_use {
            Linker::create_link(&dest, &config.alias_default())?;
        }

        Ok(())
    }
}
