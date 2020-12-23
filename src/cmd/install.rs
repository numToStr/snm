use crate::config::Config;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Install {
    #[clap(name = "version")]
    version: String,
}

impl super::Command for Install {
    type InitResult = ();

    fn init(&self, _: Config) -> anyhow::Result<Self::InitResult> {
        Ok(())
    }
}
