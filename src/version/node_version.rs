use serde::Deserialize;

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