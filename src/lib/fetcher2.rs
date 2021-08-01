use crate::fetcher::{Lts, Release};
use url::Url;

use super::SnmRes;

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
}
