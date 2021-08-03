// use std::io;
// use std::path::Path;
//
// /// Unix -- Create a symlink from destination path to the given path
// // #[cfg(unix)]
// // pub fn symlink_to<P: AsRef<Path>>(from: P, to: P) -> io::Result<()> {
// //     remove_symlink(&to)?;
// //     std::os::unix::fs::symlink(from, to)
// // }
//
// #[cfg(unix)]
// pub fn remove_symlink<P: AsRef<Path>>(path: P) -> io::Result<()> {
//     if path.as_ref().exists() {
//         std::fs::remove_file(path)?
//     }
//
//     Ok(())
// }
//
// /// Win -- Create a symlink from destination path to the given path
// // #[cfg(windows)]
// // pub fn symlink_to<P: AsRef<Path>>(from: P, to: P) -> io::Result<()> {
// //     remove_symlink(&to)?;
// //     std::os::windows::fs::symlink_dir(from, to)
// // }
//
// #[cfg(windows)]
// pub fn remove_symlink<P: AsRef<Path>>(path: P) -> io::Result<()> {
//     if path.as_ref().exists() {
//         std::fs::remove_dir(path)?
//     }
//
//     Ok(())
// }
