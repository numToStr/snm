use crate::config::Config;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Install {
    #[clap(name = "version")]
    version: String,
}

impl super::Command for Install {
    fn init(&self, _: Config) {}
}
