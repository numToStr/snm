use crate::alias::Alias;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum NodeVersion {
    Semver(semver::Version),
    Lts(String),
    Alias(String),
}

fn is_numeric(version: &str) -> bool {
    version.chars().next().unwrap().is_numeric()
}

impl NodeVersion {
    pub fn parse(version: &str) -> anyhow::Result<Self> {
        let lower_case = version.to_lowercase();
        let trimmed = lower_case.trim_start_matches("v");

        if trimmed.starts_with("lts-") || trimmed.starts_with("lts/") {
            Ok(Self::Lts(trimmed[4..].to_string()))
        } else if is_numeric(&trimmed) {
            let ver = semver::Version::parse(&trimmed)?;
            Ok(Self::Semver(ver))
        } else {
            Ok(Self::Alias(trimmed.to_string()))
        }
    }

    pub fn version_str(&self) -> String {
        format!("{}", self)
    }

    pub fn list_versions<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<NodeVersion>> {
        let mut versions = Vec::<NodeVersion>::new();
        let dirs = std::fs::read_dir(&path)?;

        for dir in dirs {
            let dir = dir?.path();
            let ver = dir.strip_prefix(&path)?.to_str();
            let ver = NodeVersion::parse(ver.unwrap())?;
            versions.push(ver);
        }

        // Sort in decreasing order;
        versions.sort_by(|a, b| b.cmp(a));

        Ok(versions)
    }

    pub fn list_aliases<'a>(&self, aliases: &'a Vec<Alias>) -> Vec<&'a Alias> {
        aliases
            .into_iter()
            .filter(|&alias| alias.version_str() == self.version_str())
            .collect()
    }

    // pub fn list_aliases_str<'a>(&self, aliases: &'a Vec<Alias>) -> Vec<&'a str> {
    //     self.list_aliases(aliases)
    //         .into_iter()
    //         .map(|verion| verion.name())
    //         .collect()
    // }
}

impl std::fmt::Display for NodeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lts(lts) => write!(f, "lts-{}", lts),
            Self::Semver(semver) => write!(f, "v{}", semver),
            Self::Alias(alias) => write!(f, "{}", alias),
        }
    }
}

impl<'de> Deserialize<'de> for NodeVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version_str = String::deserialize(deserializer)?;

        NodeVersion::parse(&version_str).map_err(serde::de::Error::custom)
    }
}
