use clap::Clap;
use dirs_next::home_dir;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

#[derive(Clap, Debug)]
pub struct Config {
    #[clap(hidden = true)]
    pub base_dir: Option<PathBuf>,

    /// Only downloads the binary
    #[clap(short, long)]
    pub download_only: bool,
}

impl Config {
    // pub fn new() -> Self {
    //     Config { base_dir: None }
    // }

    pub fn ensure_create<P: AsRef<Path>>(&self, path: P) -> P {
        create_dir_all(&path).ok();
        path
    }

    pub fn base_dir(&self) -> PathBuf {
        self.ensure_create(
            (self.base_dir.clone())
                .unwrap_or_else(|| home_dir().expect("Can't get home directory.").join(".snm")),
        )
    }

    pub fn release_dir(&self) -> PathBuf {
        self.ensure_create(self.base_dir().join("releases"))
    }

    pub fn aliases(&self) -> PathBuf {
        self.ensure_create(self.base_dir().join("aliases"))
    }
}
