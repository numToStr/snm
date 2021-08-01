use crate::config::Config;
use crate::lib::fetcher2::Fetcher2;
use crate::lib::version::user_version::UserVersion;

use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct LsRemote {
    /// Version that needs to be searched. Can be a partial semver string.
    version: Option<UserVersion>,

    /// Number of result to be shown
    #[clap(short, long, default_value = "20", conflicts_with = "all")]
    count: usize,

    /// Show all the results that matches the version
    #[clap(short, long)]
    all: bool,
}

impl super::Command for LsRemote {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        if !self.all {
            println!("-- Displaying {} results --", self.count)
        }

        let fetcher = Fetcher2::fetch(&config.dist_mirror)?;

        let releases = match &self.version {
            Some(v) => fetcher.find_releases(v),
            _ => fetcher.get_all(),
        };

        if let (true, Some(v)) = (releases.is_empty(), &self.version) {
            anyhow::bail!("No releases found with the version {:?}", v);
        }

        let releases = releases.into_iter();

        if self.all {
            releases.for_each(|release| println!("- {}", release.version));
        } else {
            releases
                .take(self.count)
                .for_each(|release| println!("- {}", release.version));
        };

        Ok(())
    }
}
