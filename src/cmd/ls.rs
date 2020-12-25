use crate::alias::Alias;
use crate::config::Config;
use crate::version::NodeVersion;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Ls;

impl super::Command for Ls {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let path = config.release_dir();
        let aliases = Alias::list(config.alias_dir())?;

        for version in NodeVersion::list_versions(path)?.iter() {
            let found = version.list_aliases(&aliases);

            println!("- {}\t{}", version, found.join(", "));
        }

        Ok(())
    }
}
