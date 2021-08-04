use crate::version::{dist_version::DistVersion, ParseVersion};
use std::{
    collections::HashMap,
    fs::read_dir,
    path::{Path, PathBuf},
};

use super::SnmRes;

pub struct Linker;

impl Linker {
    pub fn remove_link(path: &Path) -> SnmRes<()> {
        if path.exists() {
            #[cfg(unix)]
            std::fs::remove_file(path)?;

            #[cfg(windows)]
            std::fs::remove_dir(path)?;
        }

        Ok(())
    }

    pub fn create_link(original: &Path, link: &Path) -> SnmRes<()> {
        Self::remove_link(link)?;

        #[cfg(unix)]
        std::os::unix::fs::symlink(original, link)?;

        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(original, link)?;

        Ok(())
    }

    pub fn read_link(path: &Path) -> SnmRes<PathBuf> {
        std::fs::read_link(path).map_err(anyhow::Error::new)
    }

    pub fn read_convert_to_dist(alias_dir: &Path, release_dir: &Path) -> SnmRes<DistVersion> {
        let linked = Self::read_link(alias_dir)?;
        let link_ver = linked
            .strip_prefix(release_dir)?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("WTF"))?;

        DistVersion::parse(link_ver)
    }

    pub fn list_aliases(
        alias_dir: &Path,
        release_dir: &Path,
    ) -> SnmRes<HashMap<DistVersion, Vec<String>>> {
        let mut aliases: HashMap<DistVersion, Vec<String>> = HashMap::new();
        let entries = read_dir(&alias_dir)?;

        for entry in entries {
            let entry = entry?.path();
            let link = Self::read_link(&entry)?;

            let link_ver = link.strip_prefix(release_dir)?;
            let alias = entry.strip_prefix(alias_dir)?;

            if let (Some(v), Some(a)) = (link_ver.to_str(), alias.to_str()) {
                let dist_ver = DistVersion::parse(v)?;

                aliases
                    .entry(dist_ver)
                    .and_modify(|curr| curr.push(a.to_string()))
                    .or_insert_with(|| vec![a.to_string()]);
            }
        }

        Ok(aliases)
    }

    pub fn list_for_version(
        version: &DistVersion,
        alias_dir: &Path,
        release_dir: &Path,
    ) -> SnmRes<Vec<String>> {
        let mut aliases: Vec<String> = vec![];
        let entries = read_dir(&alias_dir)?;

        for entry in entries {
            let entry = entry?.path();
            let link = Self::read_link(&entry)?;

            let link_ver = link.strip_prefix(release_dir)?;
            let alias = entry.strip_prefix(alias_dir)?;

            if let (Some(v), Some(a)) = (link_ver.to_str(), alias.to_str()) {
                if DistVersion::parse(v)?.eq(version) {
                    aliases.push(a.to_string());
                }
            }
        }

        Ok(aliases)
    }
}
