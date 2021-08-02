use crate::config::Config;
use crate::lib::version::{dist_version::DistVersion, user_version::UserVersion};
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Which {
    /// Can be a partial semver string.
    version: UserVersion,
}

impl super::Command for Which {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let release_dir = config.release_dir();

        let versions = DistVersion::match_versions(&release_dir, &self.version)?;

        if versions.len() == 1 {
            if let Some(v) = versions.first() {
                println!("{}", v);
            }
        } else {
            for ver in versions {
                let bin_path = super::bin_path(release_dir.join(ver.to_string())).join("node");

                println!("- {}\t{}", ver, bin_path.display())
            }
        }

        Ok(())
    }
}
