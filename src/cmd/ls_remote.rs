use crate::config::Config;
use crate::fetcher::Fetcher;
use crate::version::Version;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct LsRemote {
    /// Version that needs to be searched
    version: Option<Version>,

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

        let releases = Fetcher::fetch(&config.dist_mirror)?;

        let (releases, version) = match &self.version {
            Some(v) => (releases.find_releases(v), Some(v)),
            _ => (releases.list, None),
        };

        if releases.is_empty() {
            println!("No releases found with the version {}", version.unwrap());
            return Ok(());
        }

        let releases = releases.into_iter();

        if self.all {
            releases.for_each(|v| println!("{}", v.version));
        } else {
            releases
                .take(self.count)
                .for_each(|v| println!("{}", v.version));
        };

        Ok(())
    }
}
