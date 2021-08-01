// use crate::symlink::symlink_to;
// use crate::{archive::Archive, progress_bar::Bar};
// use crate::{config::Config, url};
// use crate::{fetcher::Release, progress_bar::Spinner};
// use colored::*;
// use indicatif::HumanBytes;
// use std::path::{Path, PathBuf};
//
// pub struct Downloader<'a> {
//     release: &'a Release,
//     config: &'a Config,
//     release_dir: PathBuf,
// }
//
// impl<'a> Downloader<'a> {
//     pub fn new(release: &'a Release, config: &'a Config) -> Self {
//         Self {
//             release,
//             config,
//             release_dir: config.release_dir(),
//         }
//     }
//
//     pub fn download(&self, spnr: &Spinner) -> anyhow::Result<PathBuf> {
//         let v = &self.release.version;
//         let v_str = v.to_string();
//
//         let dest = &self.release_dir.join(&v_str);
//
//         if dest.exists() {
//             anyhow::bail!("Version {} is already exists locally", &v_str.bold());
//         }
//
//         let dist = url::release(&self.config.dist_mirror, v);
//
//         spnr.update_msg("Checking version...".to_string());
//
//         let res = ureq::get(&dist.url).call()?;
//
//         spnr.stop();
//
//         let len = res
//             .header("Content-Length")
//             .and_then(|x| x.parse::<u64>().ok());
//
//         let size = match len {
//             Some(l) => HumanBytes(l).to_string(),
//             None => "unknown".into(),
//         };
//
//         println!("Version   : {}", v_str.bold());
//         println!("Download  : {}", dist.url.bold());
//         println!("Size      : {}", size.bold());
//
//         println!();
//
//         let buf = Bar::new(len).read_start(res.into_reader())?;
//
//         println!();
//
//         let dest = self.install(buf, dist.name)?;
//
//         Ok(dest)
//     }
//
//     fn install(&self, buf: Vec<u8>, dist: String) -> anyhow::Result<PathBuf> {
//         let release_dir = &self.release_dir;
//         let dest = release_dir.join(&self.release.version.to_string());
//
//         Archive::new(buf).extract_into(&release_dir)?;
//
//         std::fs::rename(&release_dir.join(&dist), &dest)?;
//
//         println!("Installed : {}", &dest.display().to_string().bold());
//
//         Ok(dest)
//     }
//
//     pub fn alias_to_default(&self, dest: &Path) -> anyhow::Result<()> {
//         symlink_to(&dest, &self.config.alias_default().as_path())?;
//
//         println!();
//         println!(
//             "-- Using version {} --",
//             self.release.version.to_string().bold()
//         );
//
//         Ok(())
//     }
// }
//
// // // Conflicting with cross in CI
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //     use crate::fetcher::Lts;
// //     use crate::version::*;
// //
// //     #[test]
// //     fn download_test() -> anyhow::Result<()> {
// //         let config = Config::default();
// //         let release = Release {
// //             version: NodeVersion::parse("10.20.0").unwrap(),
// //             lts: Lts::Yes("Dubnium".to_string()),
// //         };
// //         let dir = config.release_dir();
// //         let download_path_expected = dir.join(release.version.to_string());
// //         let download_path_result = {
// //             let dwnld = Downloader::new(&release, &config);
// //             dwnld.download()?
// //         };
// //
// //         assert_eq!(download_path_expected, download_path_result);
// //
// //         std::fs::remove_dir_all(dir).unwrap();
// //         std::fs::remove_dir_all(config.alias_dir()).unwrap();
// //
// //         Ok(())
// //     }
// // }
