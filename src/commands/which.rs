use crate::cli::Config;
use clap::Clap;
use snm_core::{
    linker::Linker,
    version::{DistVersion, UserVersion},
    SnmRes,
};

#[cfg(unix)]
const EXT: &str = "node";

#[cfg(windows)]
const EXT: &str = "node.exe";

#[derive(Debug, Clap)]
pub struct Which {
    /// Can be partial or full semver string.
    version: Option<UserVersion>,
}

impl super::Command for Which {
    fn init(self, config: Config) -> SnmRes<()> {
        let r_dir = config.release_dir();

        if let Some(version) = self.version {
            let versions = DistVersion::match_versions(&r_dir, &version)?;

            if versions.len() == 1 {
                if let Some(ver) = versions.first() {
                    let bin_path = config
                        .bin_path(r_dir.join(ver.to_string()).as_ref())
                        .join(EXT);

                    println!("{}", bin_path.display());
                }
            } else {
                for ver in versions {
                    let bin_path = config
                        .bin_path(r_dir.join(ver.to_string()).as_ref())
                        .join(EXT);

                    println!("- {} \t{}", ver, bin_path.display())
                }
            }
        } else {
            let alias_default = config.alias_default();

            let dist_ver = Linker::read_convert_to_dist(&alias_default, &r_dir)?;

            let bin_path = config
                .bin_path(r_dir.join(dist_ver.to_string()).as_ref())
                .join(EXT);

            println!("{}", bin_path.display());
        }

        Ok(())
    }
}
