use crate::version::{NodeVersion, Version};
use colored::*;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Lts {
    No(bool),
    Yes(String),
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub version: NodeVersion,
    pub lts: Lts,
    // pub date: String,
    // pub files: Vec<String>,
}

pub struct Fetcher {
    pub list: Vec<Release>,
}

impl Fetcher {
    pub fn fetch(base_url: &Url) -> anyhow::Result<Self> {
        let list: Vec<Release> = ureq::get(crate::url::index(base_url).as_str())
            .call()?
            .into_json()?;

        Ok(Self { list })
    }

    pub fn lts(&self) -> anyhow::Result<&Release> {
        self.list
            .iter()
            .find(|x| match x.lts {
                Lts::Yes(_) => true,
                _ => false,
            })
            .ok_or_else(|| anyhow::anyhow!("Unable to find {} release", "lts".bold()))
    }

    pub fn lts_name(self, lts: &str) -> Option<Release> {
        self.list.into_iter().find(|x| match &x.lts {
            Lts::Yes(raw_lts) => raw_lts.to_lowercase() == lts.to_lowercase(),
            _ => false,
        })
    }

    pub fn latest(&self) -> anyhow::Result<&Release> {
        self.list
            .iter()
            .find(|x| match x.lts {
                Lts::No(_) => true,
                _ => false,
            })
            .ok_or_else(|| anyhow::anyhow!("Unable to find {} release", "latest".bold()))
    }

    pub fn find_releases(self, version: &Version) -> Vec<Release> {
        self.list
            .into_iter()
            .filter(|v| version.match_node_version(&v.version))
            .collect()
    }

    pub fn find_release(self, version: &Version) -> Option<Release> {
        self.list
            .into_iter()
            .find(|v| version.match_node_version(&v.version))
    }
}
