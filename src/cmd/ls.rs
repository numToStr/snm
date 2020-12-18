use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Ls;

impl Ls {
    pub fn init(&self) -> String {
        String::from("Ls")
    }
}
