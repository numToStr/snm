use crate::cli::Config;
use snm_core::{
    downloader::Downloader,
    fetcher::{Fetcher, Lts},
    linker::Linker,
    types::UserLts,
    version::user_version::UserVersion,
    SnmRes,
};

use clap::Clap;
use console::style;

#[derive(Debug, Clap)]
pub struct Install {
    /// A version string. Can be a partial semver or a LTS version name by the format lts/NAME
    version: UserVersion,
}

impl super::Command for Install {
    fn init(self, config: Config) -> SnmRes<()> {
        if let UserVersion::Alias(x) = self.version {
            anyhow::bail!("Unable to install version: {}", style(x).bold())
        }

        let fetcher = Fetcher::fetch(&config.dist_mirror)?;

        let release = fetcher.find_release(&self.version).ok_or_else(|| {
            anyhow::anyhow!(
                "No release found with version {}",
                style(self.version).bold()
            )
        })?;

        let dwnldr = Downloader::new(&config.dist_mirror, &release.version);

        let dwnld_dir = dwnldr.download(&config.release_dir(), &config.download_dir())?;

        if let Lts::Yes(lts) = release.lts {
            let lts = format!("{}{}", UserLts::PREFIX, lts);

            Linker::create_link(&dwnld_dir, &config.alias_dir().join(&lts))?;

            println!("Alias     : {}", style(lts).bold());
        }

        if !config.no_use {
            Linker::create_link(&dwnld_dir, &config.alias_default())?;
            println!();
            println!("Using version {}", style(&release.version).bold());
        }

        Ok(())
    }
}
