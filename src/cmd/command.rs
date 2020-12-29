use crate::config::Config;
use anyhow::Result;
use std::path::PathBuf;

pub trait Command {
    type InitResult;

    fn init(&self, config: Config) -> Result<Self::InitResult>;
}

pub fn bin_path(path: PathBuf) -> PathBuf {
    if cfg!(unix) {
        path.join("bin")
    } else {
        path
    }
}
