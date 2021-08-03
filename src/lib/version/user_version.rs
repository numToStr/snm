use serde::Deserialize;
use std::{env::current_dir, fmt::Display, fs::read_to_string, str::FromStr};

use crate::lib::{
    fetcher::{Lts, Release},
    SnmRes,
};

use super::{dist_version::DistVersion, ParseVersion};

const PACKAGE_JSON: &str = "package.json";
const NODE_FILES: [&str; 3] = [".nvmrc", ".node-version", PACKAGE_JSON];

/// `UserVersion` represents the user provided version
/// It could be alias, lts codename, partial or full semver
#[derive(Debug, PartialEq, Eq)]
pub enum UserVersion {
    /// Only major version i.e 14 | 16
    Major(u64),
    // Major and Minor both ie. 14.16 | 16.10
    MajorMinor(u64, u64),
    /// Full semver ie. 14.17.8
    Full(DistVersion),
    /// Alias name ie. latest, lts
    Alias(String),
    /// LTS codename ie. Fermium, Erbium
    /// NOTE: this should start with `lts/` or `lts-`
    Lts(String),
}

impl ParseVersion<'_> for UserVersion {
    type Item = Self;
    fn parse(ver: &str) -> SnmRes<Self::Item> {
        // Check if Lts
        if ver.starts_with("lts/") || ver.starts_with("lts-") {
            return Ok(Self::Lts(ver[4..].to_string()));
        }

        let trimmed = ver.trim_start_matches('v');

        // Check whether alias, if first char is not numeric
        if let Some(ch) = trimmed.chars().next() {
            if !ch.is_numeric() {
                return Ok(Self::Alias(ver.to_string()));
            }
        };

        let m = match DistVersion::parse(trimmed) {
            Ok(dist_version) => Self::Full(dist_version),
            Err(_) => {
                let mut s = trimmed.splitn(2, '.');

                match (s.next(), s.next()) {
                    (Some(maj), None) => Self::Major(maj.parse::<u64>()?),
                    (Some(maj), Some(min)) => {
                        Self::MajorMinor(maj.parse::<u64>()?, min.parse::<u64>()?)
                    }
                    _ => anyhow::bail!("WTF"),
                }
            }
        };

        Ok(m)
    }
}

// NOTE: this is used by `clap`
impl FromStr for UserVersion {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Display for UserVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Major(x) => write!(f, "{}.x.x", x),
            Self::MajorMinor(x, y) => write!(f, "{}.{}.x", x, y),
            Self::Full(x) => write!(f, "{}", x.to_string()),
            Self::Lts(x) => write!(f, "lts-{}", x),
            Self::Alias(x) => f.write_str(x.as_str()),
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
            (Self::Full(a), b, _) => a == b,
            (Self::MajorMinor(maj, min), DistVersion(x), _) => *maj == x.major && *min == x.minor,
            (Self::Major(a), DistVersion(b), _) => *a == b.major,
            (Self::Lts(a), _, Lts::Yes(b)) => a.to_lowercase() == b.to_lowercase(),
            // FIXME: add wildcard semver
            // (Self::Alias(_), _) => false,
            _ => false,
        }
    }

    pub fn from_file() -> SnmRes<Self> {
        let pwd = current_dir()?;

        for f_name in NODE_FILES {
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
