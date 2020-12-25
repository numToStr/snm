use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Alias {
    pub alias_path: PathBuf,
    pub dest_path: PathBuf,
}

impl Alias {
    pub fn new(alias_path: PathBuf, dest_path: PathBuf) -> Self {
        Self {
            alias_path,
            dest_path,
        }
    }

    pub fn list<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Self>> {
        let dirs = std::fs::read_dir(&path)?;
        let mut aliases = Vec::<Alias>::new();

        for alias in dirs {
            let alias_path = alias?.path();
            let dest_path = std::fs::read_link(&alias_path)?;
            aliases.push(Self::new(alias_path, dest_path))
        }

        Ok(aliases)
    }

    pub fn remove_alias(&self) -> anyhow::Result<()> {
        crate::directory::remove_symlink(&self.alias_path)?;
        Ok(())
    }

    pub fn name(&self) -> &str {
        self.alias_path.file_name().unwrap().to_str().unwrap()
    }

    pub fn version_str(&self) -> &str {
        self.dest_path.file_name().unwrap().to_str().unwrap()
    }
}
