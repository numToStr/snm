use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Latest;

impl Latest {
    pub fn init(&self) {}
}
