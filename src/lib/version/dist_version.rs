use std::fmt::Display;

use semver::Version;
use serde::Deserialize;

use crate::lib::SnmRes;

use super::ParseVersion;

/// `DistVersion` represents full semver range according to the node release
#[derive(Debug, PartialEq, Eq)]
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
