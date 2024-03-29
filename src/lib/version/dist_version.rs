use std::{fmt::Display, fs::read_dir};

use console::style;
use semver::Version;
use serde::Deserialize;

use crate::{
    fetcher::{Lts, Release},
    types::ReleaseDir,
    SnmRes,
};

use super::{ParseVersion, UserVersion};

/// `DistVersion` represents full semver range according to the node release
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DistVersion(pub(super) Version);

impl ParseVersion for DistVersion {
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
        self.0.fmt(f)
    }
}

impl AsRef<Version> for DistVersion {
    fn as_ref(&self) -> &Version {
        &self.0
    }
}

impl DistVersion {
    /// To list all the installed versions
    pub fn list_versions(release_dir: &ReleaseDir) -> SnmRes<Vec<Self>> {
        let mut versions: Vec<Self> = vec![];

        let entries = read_dir(release_dir.as_ref())?;

        for entry in entries {
            let entry = entry?.path();
            let entry = entry.strip_prefix(release_dir.as_ref())?;

            if let Some(e) = entry.to_str() {
                let dist_ver = Self::parse(e)?;

                versions.push(dist_ver);
            }
        }

        Ok(versions)
    }

    /// To match multiple installed versions with a provided user version
    pub fn match_versions(r_dir: &ReleaseDir, version: &UserVersion) -> SnmRes<Vec<Self>> {
        let mut versions: Vec<Self> = vec![];

        let entries = read_dir(r_dir.as_ref())?;

        for entry in entries {
            let entry = entry?.path();
            let entry = entry.strip_prefix(r_dir.as_ref())?;

            if let Some(e) = entry.to_str() {
                let dist_ver = Self::parse(e)?;

                let release = Release {
                    version: dist_ver,
                    lts: Lts::No,
                };

                let is_match = version.match_release(&release);

                if is_match {
                    versions.push(release.version);
                }
            }
        }

        // Sorting the list descending order
        versions.sort_by(|a, b| b.cmp(a));

        Ok(versions)
    }

    /// To match a installed version with the user provided version
    pub fn match_version(r_dir: &ReleaseDir, version: &UserVersion) -> SnmRes<Self> {
        let versions = Self::match_versions(r_dir, version)?;

        // NOTE: version list is already sorted, so I am returning the first element
        let max = versions.into_iter().next().ok_or_else(|| {
            anyhow::anyhow!("Version {} not found locally", style(version).bold())
        })?;

        Ok(max)
    }
}
