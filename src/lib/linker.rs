use crate::{
    types::{AliasDir, ReleaseDir},
    version::{DistVersion, ParseVersion},
};
use std::{
    collections::HashMap,
    fs::{read_dir, read_link},
};

use super::SnmRes;

pub struct Linker;

impl Linker {
    pub fn remove_link(path: &AliasDir) -> SnmRes<()> {
        if path.as_ref().exists() {
            #[cfg(unix)]
            std::fs::remove_file(path.as_ref())?;

            #[cfg(windows)]
            std::fs::remove_dir(path.as_ref())?;
        }

        Ok(())
    }

    pub fn create_link(original: &ReleaseDir, link: &AliasDir) -> SnmRes<()> {
        Self::remove_link(link)?;

        #[cfg(unix)]
        std::os::unix::fs::symlink(original.as_ref(), link.as_ref())?;

        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(original.as_ref(), link.as_ref())?;

        Ok(())
    }

    pub fn read_convert_to_dist(a_dir: &AliasDir, r_dir: &ReleaseDir) -> SnmRes<DistVersion> {
        let linked = read_link(a_dir.as_ref())?;
        let link_ver = linked
            .strip_prefix(r_dir.as_ref())?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Unable to read and convert alias"))?;

        DistVersion::parse(link_ver)
    }

    pub fn list_aliases(
        a_dir: &AliasDir,
        r_dir: &ReleaseDir,
    ) -> SnmRes<HashMap<DistVersion, Vec<String>>> {
        let mut aliases: HashMap<DistVersion, Vec<String>> = HashMap::new();
        let entries = read_dir(&a_dir.as_ref())?;

        for entry in entries {
            let entry = entry?.path();
            let link = read_link(&entry)?;

            let link_ver = link.strip_prefix(r_dir.as_ref())?;
            let alias = entry.strip_prefix(a_dir.as_ref())?;

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
        a_dir: &AliasDir,
        r_dir: &ReleaseDir,
    ) -> SnmRes<Vec<String>> {
        let mut aliases: Vec<String> = vec![];
        let entries = read_dir(&a_dir.as_ref())?;

        for entry in entries {
            let entry = entry?.path();
            let link = read_link(&entry)?;

            let link_ver = link.strip_prefix(r_dir.as_ref())?;
            let alias = entry.strip_prefix(a_dir.as_ref())?;

            if let (Some(v), Some(a)) = (link_ver.to_str(), alias.to_str()) {
                if DistVersion::parse(v)?.eq(version) {
                    aliases.push(a.to_string());
                }
            }
        }

        Ok(aliases)
    }
}
