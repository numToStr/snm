use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct LsRemote {
    /// The version that needs to be searched
    #[clap()]
    version: String,

    /// Show only this number of results that matches the version
    #[clap(short, default_value = "20")]
    length: String,

    /// Show all the results that matches the version
    #[clap(short, long)]
    all: bool,
}

impl LsRemote {
    pub fn init(&self) {}
}
