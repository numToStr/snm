use crate::config::Config;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Remove {
    #[clap(name = "version")]
    version: String,
}

impl super::Command for Remove {
    fn init(&self, _: Config) {}
}
