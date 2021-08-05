use std::str::FromStr;

use semver::VersionReq;
use snm_core::{types::UserLts, version::UserVersion};

#[test]
fn only_major() {
    let ver = UserVersion::from_str("10").unwrap();

    assert_eq!(
        ver,
        UserVersion::Semver(VersionReq::from_str("10").unwrap())
    );
    assert_ne!(ver, UserVersion::Lts(UserLts::new("nope")));
}

#[test]
fn major_minor() {
    let ver = UserVersion::from_str("10.15").unwrap();

    assert_eq!(
        ver,
        UserVersion::Semver(VersionReq::from_str("10.15").unwrap())
    );
    assert_ne!(ver, UserVersion::Lts(UserLts::new("nope")));
}

// #[test]
// fn match_full_version() {
//     let ver = DistVersion::parse("10.15.0").unwrap();
//     assert!(UserVersion::Full(ver.clone()).match_node_version(&ver))
// }
//
// #[test]
// fn match_major_version() {
//     let ver = DistVersion::parse("10.15.0").unwrap();
//     assert!(UserVersion::Major(10).match_node_version(&ver))
// }
//
// #[test]
// fn not_match_major_version() {
//     let ver = DistVersion::parse("10.15.0").unwrap();
//     assert!(!UserVersion::Major(19).match_node_version(&ver))
// }
//
// #[test]
// fn match_major_minor_version() {
//     let ver = DistVersion::parse("10.15.0").unwrap();
//     assert!(UserVersion::MajorMinor(10, 15).match_node_version(&ver))
// }
//
// #[test]
// fn not_match_major_minor_version() {
//     let ver = DistVersion::parse("10.15.0").unwrap();
//     assert!(!UserVersion::MajorMinor(10, 19).match_node_version(&ver))
// }
//
// #[test]
// fn major_to_version() {
//     let expected = DistVersion::parse("6.1.0").unwrap();
//     let versions = vec![
//         DistVersion::parse("6.0.0").unwrap(),
//         DistVersion::parse("6.0.1").unwrap(),
//         expected.clone(),
//         DistVersion::parse("7.0.1").unwrap(),
//     ];
//     let result = UserVersion::Major(6).to_node_version(&versions);
//
//     assert_eq!(result.unwrap(), &expected);
// }
//
// #[test]
// fn not_major_to_version() {
//     let expected = DistVersion::parse("6.1.0").unwrap();
//     let versions = vec![
//         DistVersion::parse("6.0.0").unwrap(),
//         DistVersion::parse("6.0.1").unwrap(),
//         expected.clone(),
//         DistVersion::parse("6.2.0").unwrap(),
//         DistVersion::parse("7.0.1").unwrap(),
//     ];
//     let result = UserVersion::Major(6).to_node_version(&versions);
//
//     assert_ne!(result.unwrap(), &expected);
// }
//
// #[test]
// fn major_minor_to_version() {
//     let expected = DistVersion::parse("6.0.1").unwrap();
//     let versions = vec![
//         DistVersion::parse("6.0.0").unwrap(),
//         DistVersion::parse("6.1.0").unwrap(),
//         expected.clone(),
//         DistVersion::parse("7.0.1").unwrap(),
//     ];
//     let result = UserVersion::MajorMinor(6, 0).to_node_version(&versions);
//
//     assert_eq!(result.unwrap(), &expected);
// }
//
// #[test]
// fn no_major_minor_to_version() {
//     let expected = DistVersion::parse("6.0.1").unwrap();
//     let versions = vec![
//         DistVersion::parse("6.0.0").unwrap(),
//         DistVersion::parse("6.1.0").unwrap(),
//         expected.clone(),
//         DistVersion::parse("6.0.5").unwrap(),
//         DistVersion::parse("7.0.1").unwrap(),
//     ];
//     let result = UserVersion::MajorMinor(6, 0).to_node_version(&versions);
//
//     assert_ne!(result.unwrap(), &expected);
// }
//
// #[test]
// fn from_file() {
//     let file_version = UserVersion::from_file().unwrap().unwrap();
//     let expected = UserVersion::from_str("14").unwrap();
//
//     assert_eq!(file_version, expected);
// }
//
// #[test]
// fn from_file_match_node_version() {
//     let file_version = UserVersion::from_file().unwrap().unwrap();
//     let expected = DistVersion::parse("14.15.0").unwrap();
//
//     let result = file_version.match_node_version(&expected);
//
//     assert!(result);
// }
