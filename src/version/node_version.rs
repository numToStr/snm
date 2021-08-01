use serde::Deserialize;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum NodeVersion {
    SemverReq(semver::VersionReq),
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
        let trimmed = lower_case.trim_start_matches('v');

        if trimmed.starts_with("lts-") || trimmed.starts_with("lts/") {
            Ok(Self::Lts(trimmed[4..].to_string()))
        } else if is_numeric(trimmed) {
            let ver = semver::Version::parse(trimmed)?;
            Ok(Self::Semver(ver))
        } else {
            let semver = match semver::VersionReq::parse(trimmed) {
                Ok(ver) => Self::SemverReq(ver),
                Err(_) => Self::Alias(trimmed.to_string()),
            };

            Ok(semver)
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
}

impl std::fmt::Display for NodeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lts(lts) => write!(f, "lts-{}", lts),
            Self::Semver(semver) => write!(f, "v{}", semver),
            Self::Alias(alias) => write!(f, "{}", alias),
            Self::SemverReq(semver) => write!(f, "{}", semver),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let expected_semver = NodeVersion::Semver(semver::Version::parse("14.12.0").unwrap());
        let result_semver = NodeVersion::parse("14.12.0").unwrap();

        assert_eq!(result_semver, expected_semver);

        let expected_lts = NodeVersion::Lts("boron".to_string());
        let result_lts = NodeVersion::parse("lts/boron").unwrap();

        assert_eq!(result_lts, expected_lts);
    }
}
