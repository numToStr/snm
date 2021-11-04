use crate::cli::Config;
use clap::Parser;
use console::style;
use snm_core::{downloader::Downloader, fetcher::Fetcher, linker::Linker, SnmRes};

const ALIAS: &str = "latest";

#[derive(Debug, Parser)]
pub struct Latest;

impl super::Command for Latest {
    fn init(self, config: Config) -> SnmRes<()> {
        let releases = Fetcher::fetch(&config.dist_mirror)?;
        let release = releases.latest()?;

        let dwnld = Downloader::new(&config.dist_mirror, &release.version);
        let dest = dwnld.download(&config.release_dir(), &config.download_dir())?;

        Linker::create_link(&dest, &config.alias_dir().join(&ALIAS))?;

        println!("Alias     : {}", style(ALIAS).bold());

        if !config.no_use {
            Linker::create_link(&dest, &config.alias_default())?;
            println!();
            println!("Using version: {}", style(&release.version).bold());
        }

        Ok(())
    }
}
