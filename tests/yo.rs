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
