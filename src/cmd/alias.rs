use crate::config::Config;
use crate::lib::{
    linker::Linker, version::dist_version::DistVersion, version::user_version::UserVersion, SnmRes,
};

use clap::Clap;

#[derive(Debug, Clap)]
pub struct Alias {
    /// Partial semver string
    version: UserVersion,

    /// A string consist of alphanumeric digits
    alias: String,
}

impl super::Command for Alias {
    fn init(self, config: Config) -> SnmRes<()> {
        let release_dir = config.release_dir();

        let dist_version = DistVersion::match_version(&release_dir, &self.version)?;

        let link_ver = dist_version.to_string();

        let link_src = release_dir.join(&link_ver);

        Linker::create_link(&link_src, &config.alias_dir().join(&self.alias))?;

        println!("Version {} is aliased to {}", link_ver, self.alias);

        Ok(())
    }
}
