use crate::url;
use crate::version::{NodeVersion, Version};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Lts {
    No(bool),
    Yes(String),
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub version: NodeVersion,
    pub date: String,
    pub lts: Lts,
    pub files: Vec<String>,
}

pub struct Releases {
    pub list: Vec<Release>,
}

impl Releases {
    pub fn fetch() -> anyhow::Result<Self> {
        let list: Vec<Release> = ureq::get(url::index().as_str()).call()?.into_json()?;

        Ok(Releases { list })
    }

    pub fn lts(&mut self) -> anyhow::Result<&Release> {
        self.list
            .iter()
            .find(|x| match x.lts {
                Lts::Yes(_) => true,
                _ => false,
            })
            .ok_or(anyhow::Error::msg("Unable to find release"))
    }

    pub fn latest(&mut self) -> anyhow::Result<&Release> {
        self.list
            .iter()
            .find(|x| match x.lts {
                Lts::No(_) => true,
                _ => false,
            })
            .ok_or(anyhow::Error::msg("Unable to find release."))
    }

    pub fn find_releases(self, version: &Version) -> Vec<Release> {
        self.list
            .into_iter()
            .filter(|v| version.match_version(&v.version))
            .collect()
    }

    pub fn find_release(self, version: &Version) -> Option<Release> {
        self.list
            .into_iter()
            .find(|v| version.match_version(&v.version))
    }
}
