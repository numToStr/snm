use crate::config::Config;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Latest;

impl super::Command for Latest {
    type InitResult = ();

    fn init(&self, _: Config) -> anyhow::Result<Self::InitResult> {
        Ok(())
    }
}
