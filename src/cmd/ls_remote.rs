use crate::fetcher::Fetcher;
use crate::version::Version;
use crate::{config::Config, progress_bar::Spinner};
use clap::Clap;
use colored::*;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct LsRemote {
    /// Version that needs to be searched. Can be a partial semver string.
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
        let spnr = Spinner::fetch();

        let releases = Fetcher::fetch(&config.dist_mirror)?;

        let (releases, version) = match &self.version {
            Some(v) => (releases.find_releases(v), Some(v)),
            _ => (releases.list, None),
        };

        spnr.stop();

        if !self.all {
            println!("-- Displaying {} results --", self.count)
        }

        if releases.is_empty() {
            anyhow::bail!(
                "No releases found with the version {}",
                version.unwrap().to_string().bold()
            );
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
