// For linker, prev src/alias.rs
// // Conflicting with cross in CI
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::config::Config;
//
//     fn create_dummy_symlinks() {
//         let config = Config::default();
//         let release_dir = config.release_dir();
//         let alias_dir = config.alias_dir();
//
//         std::fs::remove_dir_all(&release_dir).unwrap();
//
//         let dirs = vec![["v8.15.0", "lts"], ["v9.0.0", "latest"]];
//
//         dirs.into_iter().for_each(|dir| {
//             let dest = release_dir.join(dir.get(0).unwrap());
//             let alias = alias_dir.join(dir.get(1).unwrap());
//             std::fs::create_dir_all(&dest).unwrap();
//             crate::symlink::symlink_to(dest, alias).unwrap();
//         })
//     }
//
//     #[test]
//     fn list_test() {
//         self::create_dummy_symlinks();
//
//         let config = Config::default();
//         let aliases = Alias::list(config.alias_dir()).unwrap();
//
//         aliases.into_iter().for_each(|alias| {
//             let path = alias.path;
//             assert!(path.exists())
//         });
//
//         std::fs::remove_dir_all(config.alias_dir()).unwrap();
//         std::fs::remove_dir_all(config.release_dir()).unwrap();
//     }
// }

// For downloader
// // Conflicting with cross in CI
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::fetcher::Lts;
//     use crate::version::*;
//
//     #[test]
//     fn download_test() -> anyhow::Result<()> {
//         let config = Config::default();
//         let release = Release {
//             version: NodeVersion::parse("10.20.0").unwrap(),
//             lts: Lts::Yes("Dubnium".to_string()),
//         };
//         let dir = config.release_dir();
//         let download_path_expected = dir.join(release.version.to_string());
//         let download_path_result = {
//             let dwnld = Downloader::new(&release, &config);
//             dwnld.download()?
//         };
//
//         assert_eq!(download_path_expected, download_path_result);
//
//         std::fs::remove_dir_all(dir).unwrap();
//         std::fs::remove_dir_all(config.alias_dir()).unwrap();
//
//         Ok(())
//     }
// }

// For Fetcher
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

// For UserVersion
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::str::FromStr;
//
//     #[test]
//     fn only_major() {
//         let ver = Version::from_str("10").unwrap();
//         assert_eq!(ver, Version::Major(10))
//     }
//
//     #[test]
//     fn not_major() {
//         let ver = Version::from_str("10.15").unwrap();
//         assert_ne!(ver, Version::Major(10))
//     }
//
//     #[test]
//     fn major_minor() {
//         let ver = Version::from_str("10.15").unwrap();
//         assert_eq!(ver, Version::MajorMinor(10, 15))
//     }
//
//     #[test]
//     fn not_major_minor() {
//         let ver = Version::from_str("10").unwrap();
//         assert_ne!(ver, Version::MajorMinor(10, 10))
//     }
//
//     #[test]
//     fn match_full_version() {
//         let ver = NodeVersion::parse("10.15.0").unwrap();
//         assert!(Version::Full(ver.clone()).match_node_version(&ver))
//     }
//
//     #[test]
//     fn match_major_version() {
//         let ver = NodeVersion::parse("10.15.0").unwrap();
//         assert!(Version::Major(10).match_node_version(&ver))
//     }
//
//     #[test]
//     fn not_match_major_version() {
//         let ver = NodeVersion::parse("10.15.0").unwrap();
//         assert!(!Version::Major(19).match_node_version(&ver))
//     }
//
//     #[test]
//     fn match_major_minor_version() {
//         let ver = NodeVersion::parse("10.15.0").unwrap();
//         assert!(Version::MajorMinor(10, 15).match_node_version(&ver))
//     }
//
//     #[test]
//     fn not_match_major_minor_version() {
//         let ver = NodeVersion::parse("10.15.0").unwrap();
//         assert!(!Version::MajorMinor(10, 19).match_node_version(&ver))
//     }
//
//     #[test]
//     fn major_to_version() {
//         let expected = NodeVersion::parse("6.1.0").unwrap();
//         let versions = vec![
//             NodeVersion::parse("6.0.0").unwrap(),
//             NodeVersion::parse("6.0.1").unwrap(),
//             expected.clone(),
//             NodeVersion::parse("7.0.1").unwrap(),
//         ];
//         let result = Version::Major(6).to_node_version(&versions);
//
//         assert_eq!(result.unwrap(), &expected);
//     }
//
//     #[test]
//     fn not_major_to_version() {
//         let expected = NodeVersion::parse("6.1.0").unwrap();
//         let versions = vec![
//             NodeVersion::parse("6.0.0").unwrap(),
//             NodeVersion::parse("6.0.1").unwrap(),
//             expected.clone(),
//             NodeVersion::parse("6.2.0").unwrap(),
//             NodeVersion::parse("7.0.1").unwrap(),
//         ];
//         let result = Version::Major(6).to_node_version(&versions);
//
//         assert_ne!(result.unwrap(), &expected);
//     }
//
//     #[test]
//     fn major_minor_to_version() {
//         let expected = NodeVersion::parse("6.0.1").unwrap();
//         let versions = vec![
//             NodeVersion::parse("6.0.0").unwrap(),
//             NodeVersion::parse("6.1.0").unwrap(),
//             expected.clone(),
//             NodeVersion::parse("7.0.1").unwrap(),
//         ];
//         let result = Version::MajorMinor(6, 0).to_node_version(&versions);
//
//         assert_eq!(result.unwrap(), &expected);
//     }
//
//     #[test]
//     fn no_major_minor_to_version() {
//         let expected = NodeVersion::parse("6.0.1").unwrap();
//         let versions = vec![
//             NodeVersion::parse("6.0.0").unwrap(),
//             NodeVersion::parse("6.1.0").unwrap(),
//             expected.clone(),
//             NodeVersion::parse("6.0.5").unwrap(),
//             NodeVersion::parse("7.0.1").unwrap(),
//         ];
//         let result = Version::MajorMinor(6, 0).to_node_version(&versions);
//
//         assert_ne!(result.unwrap(), &expected);
//     }
//
//     #[test]
//     fn from_file() {
//         let file_version = Version::from_file().unwrap().unwrap();
//         let expected = Version::from_str("14").unwrap();
//
//         assert_eq!(file_version, expected);
//     }
//
//     #[test]
//     fn from_file_match_node_version() {
//         let file_version = Version::from_file().unwrap().unwrap();
//         let expected = NodeVersion::parse("14.15.0").unwrap();
//
//         let result = file_version.match_node_version(&expected);
//
//         assert!(result);
//     }
// }

// For DistVersion
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn parse_test() {
//         let expected_semver = NodeVersion::Semver(semver::Version::parse("14.12.0").unwrap());
//         let result_semver = NodeVersion::parse("14.12.0").unwrap();
//
//         assert_eq!(result_semver, expected_semver);
//
//         let expected_lts = NodeVersion::Lts("boron".to_string());
//         let result_lts = NodeVersion::parse("lts/boron").unwrap();
//
//         assert_eq!(result_lts, expected_lts);
//     }
// }
