// use std::path::{Path, PathBuf};
//
// pub fn pretty_path_name(path: &'_ Path) -> &'_ str {
//     path.file_name().unwrap().to_str().unwrap()
// }
//
// // pub fn sanitize(s: &str) -> String {
// //     s.replace("/", "-").replace("\\", "-")
// // }
//
// #[derive(Debug, Clone)]
// pub struct Alias {
//     pub path: PathBuf,
// }
//
// impl Alias {
//     pub fn new(path: PathBuf) -> Alias {
//         Alias { path }
//     }
//
//     pub fn list<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Self>> {
//         let dirs = std::fs::read_dir(&path)?;
//         let mut aliases = Vec::<Self>::new();
//
//         for alias in dirs {
//             let alias = alias?.path();
//             if alias.exists() {
//                 aliases.push(Self::new(alias))
//             }
//         }
//
//         Ok(aliases)
//     }
//
//     // pub fn list_for_version<P: AsRef<Path>>(
//     //     path: P,
//     //     version: &NodeVersion,
//     // ) -> anyhow::Result<Vec<Self>> {
//     //     let dirs = std::fs::read_dir(&path)?;
//     //     let mut aliases = Vec::<Self>::new();
//     //
//     //     for alias in dirs {
//     //         let alias = alias?.path();
//     //
//     //         if alias.exists() {
//     //             let alias = Self::new(alias);
//     //             let dest = alias.destination()?;
//     //
//     //             if pretty_path_name(&dest) == version.version_str() {
//     //                 aliases.push(alias)
//     //             }
//     //         }
//     //     }
//     //
//     //     Ok(aliases)
//     // }
//
//     // pub fn hashmap<P: AsRef<Path>>(path: P) -> anyhow::Result<HashMap<String, Vec<String>>> {
//     //     let list = std::fs::read_dir(&path)?;
//     //     let mut aliases: HashMap<String, Vec<String>> = HashMap::new();
//     //
//     //     for alias in list {
//     //         let alias = alias?.path();
//     //
//     //         if alias.exists() {
//     //             let alias = Self::new(alias);
//     //             let dest = alias.destination()?;
//     //
//     //             aliases
//     //                 .entry(pretty_path_name(&dest).to_string())
//     //                 .and_modify(|e| e.push(alias.name().to_string()))
//     //                 .or_insert_with(|| vec![alias.name().to_string()]);
//     //         }
//     //     }
//     //
//     //     Ok(aliases)
//     // }
//
//     pub fn destination(&self) -> anyhow::Result<PathBuf> {
//         std::fs::read_link(&self.path).map_err(anyhow::Error::new)
//     }
//
//     pub fn remove(&self) -> anyhow::Result<()> {
//         crate::symlink::remove_symlink(&self.path).map_err(anyhow::Error::new)
//     }
//
//     // pub fn name(&self) -> &str {
//     //     pretty_path_name(&self.path)
//     // }
// }
//
// // Conflicting with cross in CI
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //     use crate::config::Config;
// //
// //     fn create_dummy_symlinks() {
// //         let config = Config::default();
// //         let release_dir = config.release_dir();
// //         let alias_dir = config.alias_dir();
// //
// //         std::fs::remove_dir_all(&release_dir).unwrap();
// //
// //         let dirs = vec![["v8.15.0", "lts"], ["v9.0.0", "latest"]];
// //
// //         dirs.into_iter().for_each(|dir| {
// //             let dest = release_dir.join(dir.get(0).unwrap());
// //             let alias = alias_dir.join(dir.get(1).unwrap());
// //             std::fs::create_dir_all(&dest).unwrap();
// //             crate::symlink::symlink_to(dest, alias).unwrap();
// //         })
// //     }
// //
// //     #[test]
// //     fn list_test() {
// //         self::create_dummy_symlinks();
// //
// //         let config = Config::default();
// //         let aliases = Alias::list(config.alias_dir()).unwrap();
// //
// //         aliases.into_iter().for_each(|alias| {
// //             let path = alias.path;
// //             assert!(path.exists())
// //         });
// //
// //         std::fs::remove_dir_all(config.alias_dir()).unwrap();
// //         std::fs::remove_dir_all(config.release_dir()).unwrap();
// //     }
// // }
