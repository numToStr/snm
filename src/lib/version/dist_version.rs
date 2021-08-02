use std::{fmt::Display, fs::read_dir, path::Path};

use semver::Version;
use serde::Deserialize;

use crate::lib::{
    fetcher2::{Lts, Release},
    SnmRes,
};

use super::{user_version::UserVersion, ParseVersion};

/// `DistVersion` represents full semver range according to the node release
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DistVersion(pub Version);

impl ParseVersion<'_> for DistVersion {
    type Item = Self;
    fn parse(v: &str) -> SnmRes<Self::Item> {
        Ok(DistVersion(Version::parse(v)?))
    }
}

impl<'de> Deserialize<'de> for DistVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v: String = String::deserialize(deserializer)?;

        Self::parse(v.trim_start_matches('v')).map_err(serde::de::Error::custom)
    }
}

impl Display for DistVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl DistVersion {
    /// To list all the installed versions
    pub fn list_versions(release_dir: &Path) -> SnmRes<Vec<Self>> {
        let mut versions: Vec<Self> = vec![];

        let entries = read_dir(release_dir)?;

        for entry in entries {
            let entry = entry?.path();
            let entry = entry.strip_prefix(release_dir)?;

            if let Some(e) = entry.to_str() {
                let dist_ver = Self::parse(e)?;

                versions.push(dist_ver);
            }
        }

        Ok(versions)
    }

    /// To match a install version with the user provided version
    pub fn match_user_version(release_dir: &Path, version: &UserVersion) -> SnmRes<Self> {
        let mut versions: Vec<Self> = vec![];

        let entries = read_dir(release_dir)?;

        for entry in entries {
            let entry = entry?.path();
            let entry = entry.strip_prefix(release_dir)?;

            if let Some(e) = entry.to_str() {
                let dist_ver = Self::parse(e)?;

                let release = Release {
                    version: dist_ver,
                    lts: Lts::No(false),
                };

                let is_match = version.match_release(&release);

                if is_match {
                    versions.push(release.version);
                }
            }
        }

        let max = versions
            .into_iter()
            .max()
            .ok_or_else(|| anyhow::anyhow!("Version {:?} not found locally", version))?;

        Ok(max)
    }
}
