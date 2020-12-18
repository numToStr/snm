use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct UnInstall;

impl UnInstall {
    pub fn init(&self) -> String {
        String::from("UnInstall")
    }
}
