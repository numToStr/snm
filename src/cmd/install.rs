use crate::config::Config;
use crate::lib::{
    alias2::Alias2,
    downloader2::Downloader2,
    fetcher2::{Fetcher2, Lts},
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
    type InitResult = ();

    fn init(&self, config: Config) -> SnmRes<Self::InitResult> {
        if let UserVersion::Alias(_) = self.version {
            anyhow::bail!("Unable to install version: {:?}", self.version)
        }

        let fetcher = Fetcher2::fetch(&config.dist_mirror)?;

        let release = fetcher.find_release(&self.version).ok_or_else(|| {
            anyhow::anyhow!("No release found with the version {:?}", self.version)
        })?;

        let dwnldr = Downloader2::new(&config.dist_mirror, &release.version);

        let dwnld_dir = dwnldr.download(&config.release_dir())?;

        let linker = Alias2::new(&dwnld_dir);

        if let Lts::Yes(lts) = release.lts {
            let lts = format!("lts-{}", lts);

            linker.create_link(&config.alias_dir().join(&lts))?;

            println!("Alias     : {}", &lts);
        }

        if !config.no_use {
            linker.create_link(&config.alias_default())?;
        }

        Ok(())
    }
}
