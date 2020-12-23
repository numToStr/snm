use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Prune;

impl Prune {
    pub fn init(&self) {}
}
