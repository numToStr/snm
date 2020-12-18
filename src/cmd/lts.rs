use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Lts;

impl Lts {
    pub fn init(&self) -> String {
        String::from("Lts")
    }
}
