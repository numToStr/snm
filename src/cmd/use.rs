use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Use;

impl Use {
    pub fn init(&self) -> String {
        String::from("Use")
    }
}
