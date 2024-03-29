use crate::cli::Config;
use snm_core::{
    linker::Linker,
    types::UserAlias,
    version::{DistVersion, UserVersion},
    SnmRes,
};

use clap::Parser;
use console::style;

#[derive(Debug, Parser)]
pub struct Alias {
    /// Partial semver string
    version: UserVersion,

    /// A string consist of alphanumeric digits
    alias: UserAlias,
}

impl super::Command for Alias {
    fn init(self, config: Config) -> SnmRes<()> {
        let release_dir = config.release_dir();

        let dist_version = DistVersion::match_version(&release_dir, &self.version)?;

        let link_ver = dist_version.to_string();

        let link_src = release_dir.join(&link_ver);

        Linker::create_link(&link_src, &config.alias_dir().join(self.alias.as_ref()))?;

        println!(
            "Version {} is aliased to {}",
            style(link_ver).bold(),
            style(self.alias).bold()
        );

        Ok(())
    }
}
