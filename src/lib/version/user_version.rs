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
    /// Full, Partial or Range semver ie. 14 | 14.17 | 14.17.4 | >14.14 | <=12.3
    Semver(VersionReq),
    /// Alias name ie. latest, lts
    Alias(UserAlias),
    /// LTS codename ie. fermium, erbium
    Lts(UserLts),
}

impl ParseVersion<'_> for UserVersion {
    type Item = Self;
    fn parse(ver: &str) -> SnmRes<Self::Item> {
        // check if the version is a semver string
        let m = match VersionReq::parse(ver.trim_start_matches('v')) {
            Ok(x) => Self::Semver(x),
            Err(_) => {
                // Check if Lts, else alias
                if UserLts::is_lts(ver) {
                    Self::Lts(UserLts::new(ver))
                } else {
                    Self::Alias(UserAlias::new(ver))
                }
            }
        };

        Ok(m)
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
            Self::Semver(x) => x.fmt(f),
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
            (Self::Semver(a), DistVersion(b), _) => a.matches(b),
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
