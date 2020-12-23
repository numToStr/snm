use crate::config::Config;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct UnInstall;

impl super::Command for UnInstall {
    fn init(&self, _: Config) {}
}
