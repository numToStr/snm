use std::path::Path;

use super::SnmRes;

pub struct Alias2<'a> {
    src: &'a Path,
}

impl<'a> Alias2<'a> {
    pub fn new(src: &'a Path) -> Self {
        Self { src }
    }

    pub fn remove_link(path: &Path) -> SnmRes<()> {
        if path.exists() {
            #[cfg(unix)]
            std::fs::remove_file(path)?;

            #[cfg(windows)]
            std::fs::remove_dir(path)?;
        }

        Ok(())
    }

    pub fn create_link(&self, dest: &Path) -> SnmRes<()> {
        Self::remove_link(dest)?;

        #[cfg(unix)]
        std::os::unix::fs::symlink(self.src, dest)?;

        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(from, to)?;

        Ok(())
    }
}
