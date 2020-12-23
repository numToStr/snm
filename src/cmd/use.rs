use crate::config::Config;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Use;

impl super::Command for Use {
    fn init(&self, _: Config) {}
}
