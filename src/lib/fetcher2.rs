use serde::Deserialize;
use url::Url;

use super::{
    version::{dist_version::DistVersion, user_version::UserVersion},
    SnmRes,
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Lts {
    No(bool),
    Yes(String),
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub version: DistVersion,
    pub lts: Lts,
}

pub struct Fetcher2 {
    releases: Vec<Release>,
}

impl Fetcher2 {
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
            .find(|x| matches!(x.lts, Lts::No(_)))
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
