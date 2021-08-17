use semver::VersionReq;
use serde::Deserialize;
use std::{env::current_dir, fmt::Display, fs::read_to_string, str::FromStr};

use crate::{
    fetcher::{Lts, Release},
    types::{UserAlias, UserLts},
    SnmRes,
};

use super::{DistVersion, ParseVersion};

const PACKAGE_JSON: &str = "package.json";
const VERSION_FILES: [&str; 3] = [".nvmrc", ".node-version", PACKAGE_JSON];

/// `UserVersion` represents the user provided version
/// It could be alias, lts codename, partial or full semver
#[derive(Debug, PartialEq, Eq)]
pub enum UserVersion {
    /// Only major segment ie. 18
    Major(u64),
    /// Major and Minor segment ie. 12.3
    MajorMinor(u64, u64),
    /// Full semver ie. 14.17.4
    Semver(DistVersion),
    /// Range semver ie. >14.14 | <=12.3
    Range(VersionReq),
    /// Alias name ie. latest, lts
    Alias(UserAlias),
    /// LTS codename ie. fermium, erbium
    Lts(UserLts),
}

impl ParseVersion<'_> for UserVersion {
    type Item = Self;
    fn parse(v: &str) -> SnmRes<Self::Item> {
        let trimmed = v.trim_start_matches('v');

        let version = if let Ok(x) = DistVersion::parse(trimmed) {
            Self::Semver(x)
        } else {
            let is_numeric = trimmed.chars().next().unwrap_or_default().is_numeric();

            if is_numeric {
                let mut splitted = trimmed.splitn(2, '.');

                match (splitted.next(), splitted.next()) {
                    (Some(a), Some(b)) => Self::MajorMinor(a.parse::<u64>()?, b.parse::<u64>()?),
                    (Some(a), None) => Self::Major(a.parse::<u64>()?),
                    _ => anyhow::bail!("Unable to parse the user provided version"),
                }
            } else if let Ok(x) = VersionReq::parse(v) {
                Self::Range(x)
            } else if UserLts::is_lts(v) {
                Self::Lts(UserLts::new(v))
            } else {
                Self::Alias(UserAlias::new(v))
            }
        };

        Ok(version)
    }
}

// NOTE: this is used by `clap` to parse user input
impl FromStr for UserVersion {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Display for UserVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Major(major) => write!(f, "v{}.x.x", major),
            Self::MajorMinor(major, minor) => write!(f, "v{}.{}.x", major, minor),
            Self::Semver(x) => x.fmt(f),
            Self::Range(x) => x.fmt(f),
            Self::Alias(x) => x.fmt(f),
            Self::Lts(x) => x.fmt(f),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Engines {
    node: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PkgJson {
    engines: Option<Engines>,
}

impl UserVersion {
    pub fn match_release(&self, release: &Release) -> bool {
        match (self, &release.version, &release.lts) {
            (Self::Major(a), DistVersion(b), _) => a == &b.major,
            (Self::MajorMinor(a, b), DistVersion(c), _) => a == &c.major && b == &c.minor,
            (Self::Semver(a), b, _) => a.eq(b),
            (Self::Range(a), DistVersion(b), _) => a.matches(b),
            (Self::Lts(a), _, Lts::Yes(b)) => a.as_ref() == b,
            _ => false,
        }
    }

    pub fn from_file() -> SnmRes<Self> {
        let pwd = current_dir()?;

        for f_name in VERSION_FILES {
            let f_path = pwd.join(&f_name);

            if !f_path.exists() {
                continue;
            }

            let version_file = read_to_string(&f_path)?;

            if f_name.eq(PACKAGE_JSON) {
                let pkg_json: PkgJson = serde_json::from_str(&version_file)?;

                if let Some(Engines { node: Some(v) }) = pkg_json.engines {
                    let parsed = Self::parse(&v)?;

                    return Ok(parsed);
                }
            } else {
                let line = version_file.lines().next();

                if let Some(l) = line {
                    let parsed = Self::parse(l)?;

                    return Ok(parsed);
                }
            }
        }

        anyhow::bail!("Unable to read version from dotfiles. Please provide a version manually.")
    }
}
