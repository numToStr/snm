use crate::cli::Config;
use clap::Clap;
use snm_core::{
    version::{DistVersion, UserVersion},
    SnmRes,
};

#[cfg(unix)]
const EXT: &str = "node";

#[cfg(windows)]
const EXT: &str = "node.exe";

#[derive(Debug, Clap)]
pub struct Which {
    /// Can be a partial semver string.
    version: UserVersion,
}

impl super::Command for Which {
    fn init(self, config: Config) -> SnmRes<()> {
        let release_dir = config.release_dir();

        let versions = DistVersion::match_versions(&release_dir, &self.version)?;

        if versions.len() == 1 {
            if let Some(ver) = versions.first() {
                let bin_path = config.bin_path(release_dir.join(ver.to_string())).join(EXT);

                println!("{}", bin_path.display());
            }
        } else {
            for ver in versions {
                let bin_path = config.bin_path(release_dir.join(ver.to_string())).join(EXT);

                println!("- {} \t{}", ver, bin_path.display())
            }
        }

        Ok(())
    }
}
