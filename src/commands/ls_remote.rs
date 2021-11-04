use crate::cli::Config;
use clap::Parser;
use console::style;
use snm_core::{fetcher::Fetcher, version::UserVersion, SnmRes};

#[derive(Debug, Parser)]
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
    fn init(self, config: Config) -> SnmRes<()> {
        if !self.all {
            eprintln!("-: Displaying {} results :-", self.count);
        }

        let fetcher = Fetcher::fetch(&config.dist_mirror)?;

        let releases = match &self.version {
            Some(v) => fetcher.find_releases(v),
            _ => fetcher.get_all(),
        };

        if let (true, Some(ver)) = (releases.is_empty(), &self.version) {
            anyhow::bail!("No releases found with version {}", style(ver).bold());
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
