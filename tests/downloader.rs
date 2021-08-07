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
