use crate::version::NodeVersion;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn pretty_path_name<'a>(path: &'a PathBuf) -> &'a str {
    path.file_name().unwrap().to_str().unwrap()
}

pub fn sanitize(s: &str) -> String {
    if cfg!(unix) {
        s.replace("/", "-")
    } else {
        s.replace("\\", "-")
    }
}

#[derive(Debug, Clone)]
pub struct Alias2 {
    pub path: PathBuf,
}

impl Alias2 {
    pub fn new(path: PathBuf) -> Alias2 {
        Alias2 { path }
    }

    pub fn list<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Self>> {
        let dirs = std::fs::read_dir(&path)?;
        let mut aliases = Vec::<Self>::new();

        for alias in dirs {
            let alias = alias?.path();
            if alias.exists() {
                aliases.push(Self::new(alias))
            }
        }

        Ok(aliases)
    }

    pub fn list_for_version<P: AsRef<Path>>(
        path: P,
        version: &NodeVersion,
    ) -> anyhow::Result<Vec<Self>> {
        let dirs = std::fs::read_dir(&path)?;
        let mut aliases = Vec::<Self>::new();

        for alias in dirs {
            let alias = alias?.path();

            if alias.exists() {
                let alias = Self::new(alias);
                let dest = alias.destination()?;

                if pretty_path_name(&dest) == version.version_str() {
                    aliases.push(alias)
                }
            }
        }

        Ok(aliases)
    }

    pub fn hashmap<'a, P: AsRef<Path>>(path: P) -> anyhow::Result<HashMap<String, Vec<String>>> {
        let list = std::fs::read_dir(&path)?;
        let mut aliases: HashMap<String, Vec<String>> = HashMap::new();

        for alias in list {
            let alias = alias?.path();

            if alias.exists() {
                let alias = Self::new(alias);
                let dest = alias.destination()?;

                aliases
                    .entry(pretty_path_name(&dest).to_string())
                    .and_modify(|e| e.push(alias.name().to_string()))
                    .or_insert(vec![alias.name().to_string()]);
            }
        }

        Ok(aliases)
    }

    pub fn destination(&self) -> anyhow::Result<PathBuf> {
        std::fs::read_link(&self.path).map_err(|e| anyhow::Error::new(e))
    }

    pub fn remove(&self) -> anyhow::Result<()> {
        crate::symlink::remove_symlink(&self.path).map_err(|e| anyhow::Error::new(e))
    }

    pub fn name(&self) -> &str {
        pretty_path_name(&self.path)
    }
}
