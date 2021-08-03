use crate::config::Config;
use crate::lib::{
    downloader2::Downloader2,
    fetcher2::{Fetcher2, Lts},
    linker::Linker,
    version::user_version::UserVersion,
    SnmRes,
};

use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Install {
    /// A version string. Can be a partial semver or a LTS version name by the format lts/NAME
    version: UserVersion,
}

impl super::Command for Install {
    fn init(self, config: Config) -> SnmRes<()> {
        if let UserVersion::Alias(_) = self.version {
            anyhow::bail!("Unable to install version: {:?}", self.version)
        }

        let fetcher = Fetcher2::fetch(&config.dist_mirror)?;

        let release = fetcher.find_release(&self.version).ok_or_else(|| {
            anyhow::anyhow!("No release found with the version {:?}", self.version)
        })?;

        let dwnldr = Downloader2::new(&config.dist_mirror, &release.version);

        let dwnld_dir = dwnldr.download(&config.release_dir())?;

        if let Lts::Yes(lts) = release.lts {
            let lts = format!("lts-{}", lts);

            Linker::create_link(&dwnld_dir, &config.alias_dir().join(&lts))?;

            println!("Alias     : {}", &lts);
        }

        if !config.no_use {
            Linker::create_link(&dwnld_dir, &config.alias_default())?;
        }

        Ok(())
    }
}
