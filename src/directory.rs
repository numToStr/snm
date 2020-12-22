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
        self.remove_symlink(&to)?;
        std::os::unix::fs::symlink(&self.dest_path, to)
    }

    #[cfg(target_family = "unix")]
    fn remove_symlink<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        if path.as_ref().exists() {
            std::fs::remove_file(path)?
        }

        Ok(())
    }

    /// [[ Win ]] Create a symlink from destination path to the given path
    #[cfg(target_family = "windows")]
    pub fn symlink<P: AsRef<Path>>(&self, from: P, to: P) -> io::Result<()> {
        self.remove_symlink(&to)?;
        std::os::windows::fs::symlink_dir(from, to)
    }

    #[cfg(target_family = "windows")]
    fn remove_symlink<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        if path.as_ref().exists() {
            std::fs::remove_dir(path)?
        }

        Ok(())
    }
}
