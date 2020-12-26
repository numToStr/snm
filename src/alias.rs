use std::collections::HashMap;
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

    pub fn hashmap<'a, P: AsRef<Path>>(path: P) -> anyhow::Result<HashMap<String, Vec<String>>> {
        let dirs = std::fs::read_dir(&path)?;
        let mut aliases: HashMap<String, Vec<String>> = HashMap::with_capacity(10);

        for alias in dirs {
            let alias_path = alias?.path();
            let dest_path = std::fs::read_link(&alias_path)?;

            aliases
                .entry(pretty_path_name(&dest_path).to_string())
                .and_modify(|e| e.push(pretty_path_name(&alias_path).to_string()))
                .or_insert(vec![pretty_path_name(&alias_path).to_string()]);
        }

        Ok(aliases)
    }

    pub fn remove_alias(&self) -> anyhow::Result<()> {
        crate::symlink::remove_symlink(&self.alias_path)?;
        Ok(())
    }

    pub fn name(&self) -> &str {
        pretty_path_name(&self.alias_path)
    }

    pub fn version_str(&self) -> &str {
        pretty_path_name(&self.dest_path)
    }
}

fn pretty_path_name<'a>(path: &'a PathBuf) -> &'a str {
    path.file_name().unwrap().to_str().unwrap()
}
