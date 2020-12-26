use crate::alias::Alias;
use crate::config::Config;
use crate::version::NodeVersion;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Ls;

impl super::Command for Ls {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let aliases = Alias::list(&config.alias_dir())?;
        let versions = NodeVersion::list_versions(&config.release_dir())?;

        for version in versions.into_iter() {
            let found = version.list_aliases_str(&aliases);

            println!("- {}\t{}", version, found.join(", "));
        }

        Ok(())
    }
}
