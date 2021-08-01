use std::str::FromStr;

use crate::lib::{
    fetcher2::{Lts, Release},
    SnmRes,
};

use super::{dist_version::DistVersion, ParseVersion};

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

impl FromStr for UserVersion {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl UserVersion {
    pub fn match_release(&self, release: &Release) -> bool {
        match (self, &release.version, &release.lts) {
            (Self::Full(a), b, _) => a == b,
            (Self::MajorMinor(maj, min), DistVersion(x), _) => *maj == x.major && *min == x.minor,
            (Self::Major(a), DistVersion(b), _) => *a == b.major,
            (Self::Lts(a), _, Lts::Yes(b)) => a.to_lowercase() == b.to_lowercase(),
            // (Self::Alias(_), _) => false,
            _ => false,
        }
    }
}
