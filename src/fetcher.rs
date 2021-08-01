use crate::version::{NodeVersion, Version};
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

    pub fn lts_name(self, lts: &str) -> Option<Release> {
        self.list.into_iter().find(|x| match &x.lts {
            Lts::Yes(raw_lts) => raw_lts.to_lowercase() == lts.to_lowercase(),
            _ => false,
        })
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn fetch() -> Fetcher {
//         let config = crate::config::Config::default();
//         Fetcher::fetch(&config.dist_mirror).unwrap()
//     }
//
//     #[test]
//     fn lts_test() {
//         let list = self::fetch();
//         let lts = list.lts();
//
//         assert!(lts.is_ok());
//     }
//
//     #[test]
//     fn latest_test() {
//         let list = self::fetch();
//         let latest = list.latest();
//
//         assert!(latest.is_ok());
//     }
//
//     #[test]
//     fn find_release_test() {
//         let list = self::fetch();
//
//         let node_version = NodeVersion::parse("10.10.0").unwrap();
//         let version_semver = Version::Full(node_version.clone());
//         let release = list.find_release(&version_semver).unwrap();
//
//         assert_eq!(release.version, node_version)
//     }
//
//     #[test]
//     fn find_releases_test() {
//         let list = self::fetch();
//
//         let version = Version::Major(10);
//         let releases = list.find_releases(&version);
//
//         let semver = NodeVersion::parse("11.0.0").unwrap();
//
//         releases
//             .into_iter()
//             .for_each(|release| assert!(release.version.lt(&semver)))
//     }
//
//     #[test]
//     fn lts_name_test() {
//         let list = self::fetch();
//         let lts = list.lts_name("fermium").unwrap();
//         let version = Version::Major(14);
//
//         assert!(version.match_node_version(&lts.version));
//     }
// }
