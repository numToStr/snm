use lazy_static::lazy_static;
use semver::VersionReq;
use std::str::FromStr;

use snm_core::{
    fetcher::{Fetcher, Lts},
    version::{DistVersion, ParseVersion, UserVersion},
    MIRROR,
};
use url::Url;

lazy_static! {
    static ref FETCH: Fetcher = Fetcher::fetch(&Url::from_str(MIRROR).unwrap()).unwrap();
}

#[test]
fn lts() {
    let release = FETCH.lts().unwrap();

    assert_ne!(release.lts, Lts::No);
}

#[test]
fn latest() {
    let release = FETCH.latest().unwrap();

    assert_eq!(release.lts, Lts::No);
}

#[test]
fn find_release_semver() {
    let dist_version = DistVersion::parse("10.10.0").unwrap();
    let user_version = UserVersion::Range(VersionReq::parse("10").unwrap());
    let release = FETCH.find_release(&user_version).unwrap();

    assert_eq!(release.version.as_ref().major, dist_version.as_ref().major)
}

#[test]
fn find_release_lts() {
    // NOTE: `fermium` is the lts codename of v14.x.x
    let dist_version = VersionReq::parse("14").unwrap();
    let user_version = UserVersion::parse("lts/Fermium").unwrap();
    let release = FETCH.find_release(&user_version).unwrap();

    assert_eq!(release.lts, Lts::Yes("fermium".to_string()));
    assert!(dist_version.matches(release.version.as_ref()));
}

// ISSUE: this test is not compiling
// #[test]
// fn find_releases() {
//     let version = UserVersion::parse("10").unwrap();
//     let releases = FETCH.find_releases(&version);
//
//     let semver = VersionReq::parse("10").unwrap();
//
//     releases
//         .into_iter()
//         .for_each(|release| assert!(semver.matches(release.version.as_ref())));
// }
