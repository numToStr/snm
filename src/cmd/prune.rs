use crate::config::Config;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Prune;

impl super::Command for Prune {
    fn init(&self, _: Config) {}
}
