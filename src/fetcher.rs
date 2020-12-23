use crate::url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Lts {
    No(bool),
    Yes(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    pub version: String,
    pub date: String,
    pub lts: Lts,
    pub files: Vec<String>,
}

pub struct Releases {
    pub list: Vec<Release>,
}

impl Releases {
    pub fn fetch() -> anyhow::Result<Self> {
        let list = ureq::get(url::index().as_str())
            .call()
            .into_json_deserialize::<Vec<Release>>()?;

        Ok(Releases { list })
    }

    pub fn lts(&mut self) -> anyhow::Result<Release> {
        self.list
            .drain(..)
            .find(|x| match x.lts {
                Lts::Yes(_) => true,
                _ => false,
            })
            .ok_or(anyhow::Error::msg("Unable to find release"))
    }

    pub fn latest(&mut self) -> anyhow::Result<Release> {
        self.list
            .drain(..)
            .find(|x| match x.lts {
                Lts::No(_) => true,
                _ => false,
            })
            .ok_or(anyhow::Error::msg("Unable to find release."))
    }
}
