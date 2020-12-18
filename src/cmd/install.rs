use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Install {
    #[clap(name = "version")]
    version: String,
}

impl Install {
    pub fn init(&self) -> String {
        format!("Install {}", self.version)
    }
}
