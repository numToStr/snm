use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Remove {
    #[clap(name = "version")]
    version: String,
}

impl Remove {
    pub fn init(&self) -> String {
        format!("Remove {}", self.version)
    }
}
