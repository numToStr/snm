use serde::Deserialize;
use url::Url;

use super::{
    version::{DistVersion, UserVersion},
    SnmRes,
};

#[derive(Debug)]
pub enum Lts {
    No,
    Yes(String),
}

// NOTE:
// I had to manually implement deserializer because the lts codename is coming from api
// is in sentence case. So I am converting it to lowercase to be consistent.
impl<'de> Deserialize<'de> for Lts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = match String::deserialize(deserializer) {
            Ok(v) => Self::Yes(v.to_lowercase()),
            _ => Self::No,
        };

        Ok(val)
    }
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub version: DistVersion,
    pub lts: Lts,
}

pub struct Fetcher {
    releases: Vec<Release>,
}

impl Fetcher {
    pub fn fetch(mirror: &Url) -> SnmRes<Self> {
        let releases: Vec<Release> = ureq::get(&format!("{}/index.json", mirror))
            .call()?
            .into_json()?;

        Ok(Self { releases })
    }

    pub fn lts(&self) -> SnmRes<&Release> {
        self.releases
            .iter()
            .find(|x| matches!(x.lts, Lts::Yes(_)))
            .ok_or_else(|| anyhow::anyhow!("Unable to find {} release", "lts"))
    }

    pub fn latest(&self) -> SnmRes<&Release> {
        self.releases
            .iter()
            .find(|x| matches!(x.lts, Lts::No))
            .ok_or_else(|| anyhow::anyhow!("Unable to find {} release", "latest"))
    }

    pub fn find_release(self, version: &UserVersion) -> Option<Release> {
        self.releases
            .into_iter()
            .find(|release| version.match_release(release))
    }

    pub fn find_releases(self, version: &UserVersion) -> Vec<Release> {
        self.releases
            .into_iter()
            .filter(|v| version.match_release(v))
            .collect()
    }

    pub fn get_all(self) -> Vec<Release> {
        self.releases
    }
}
