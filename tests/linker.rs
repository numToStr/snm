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
