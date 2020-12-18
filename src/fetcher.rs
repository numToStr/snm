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
    pub fn fetch() -> Self {
        Releases {
            list: ureq::get(url::index().as_str())
                .call()
                .into_json_deserialize::<Vec<Release>>()
                .unwrap(),
        }
    }

    pub fn lts(&mut self) -> Option<Release> {
        self.list.drain(..).find(|x| match x.lts {
            Lts::Yes(_) => true,
            _ => false,
        })
    }

    // pub fn latest(&mut self) -> Option<Release> {
    //     self.list.drain(..).find(|x| match x.lts {
    //         Lts::No(_) => true,
    //         _ => false,
    //     })
    // }
}
