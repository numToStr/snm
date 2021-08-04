use semver::VersionReq;
use serde::Deserialize;
use std::{env::current_dir, fmt::Display, fs::read_to_string, str::FromStr};

use crate::{
    fetcher::{Lts, Release},
    SnmRes,
};

use super::{dist_version::DistVersion, ParseVersion};

const PACKAGE_JSON: &str = "package.json";
const VERSION_FILES: [&str; 3] = [".nvmrc", ".node-version", PACKAGE_JSON];

/// `UserVersion` represents the user provided version
/// It could be alias, lts codename, partial or full semver
#[derive(Debug, PartialEq, Eq)]
pub enum UserVersion {
    /// Full semver ie. 14.17.8
    Full(DistVersion),
    /// Partial or Range semver ie. 12 | 14.13 | >14.14 | <=12.3
    Partial(VersionReq),
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

        // check if the version is a semver string
        let m = match DistVersion::parse(trimmed) {
            Ok(dist_version) => Self::Full(dist_version),
            Err(_) => {
                // Check if the version is wildcard semver
                match VersionReq::parse(ver) {
                    Ok(x) => Self::Partial(x),
                    Err(err) => {
                        // Check whether alias, if first char is not numeric
                        if let Some(ch) = trimmed.chars().next() {
                            if !ch.is_numeric() {
                                return Ok(Self::Alias(ver.to_string()));
                            }
                        };

                        anyhow::bail!(err)
                    }
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
            Self::Full(x) => x.fmt(f),
            Self::Partial(x) => x.fmt(f),
            Self::Alias(x) => f.write_str(x.as_str()),
            Self::Lts(x) => write!(f, "lts-{}", x),
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
            (Self::Partial(a), DistVersion(b), _) => a.matches(b),
            (Self::Lts(a), _, Lts::Yes(b)) => a.to_lowercase() == b.to_lowercase(),
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
