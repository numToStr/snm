use std::io;
use std::path::{Path, PathBuf};

pub struct Directory<'a> {
    /// Downloaded path of the nodejs
    dest_path: &'a PathBuf,
}

impl<'d> Directory<'d> {
    pub fn new(path: &'d PathBuf) -> Self {
        Self { dest_path: path }
    }

    /// Rename the given path to the destination path
    pub fn rename_from<P: AsRef<Path>>(&self, from: P) -> io::Result<()> {
        std::fs::rename(from, &self.dest_path)
    }

    /// [[ Unix ]] Create a symlink from destination path to the given path
    #[cfg(target_family = "unix")]
    pub fn symlink_to<P: AsRef<Path>>(&self, to: P) -> io::Result<()> {
        std::os::unix::fs::symlink(&self.dest_path, to)
    }

    /// [[ Win ]] Create a symlink from destination path to the given path
    #[cfg(target_family = "windows")]
    pub fn symlink<P: AsRef<Path>>(&self, from: P, to: P) -> io::Result<()> {
        std::os::windows::fs::symlink(from, to)
    }
}
