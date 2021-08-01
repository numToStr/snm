// use std::{io::Cursor, path::Path};
//
// pub struct Archive {
//     reader: Cursor<Vec<u8>>,
// }
//
// impl Archive {
//     pub fn new(buf: Vec<u8>) -> Self {
//         Self {
//             reader: Cursor::new(buf),
//         }
//     }
//
//     #[cfg(unix)]
//     pub fn extract_into<P: AsRef<Path>>(self, path: P) -> anyhow::Result<()> {
//         let xz_stream = xz2::read::XzDecoder::new(self.reader);
//         let mut archive = tar::Archive::new(xz_stream);
//         archive.unpack(path).map_err(anyhow::Error::new)
//     }
//
//     #[cfg(windows)]
//     pub fn extract_into<P: AsRef<Path>>(self, path: P) -> anyhow::Result<()> {
//         use std::{fs, io};
//
//         let mut archive = zip::read::ZipArchive::new(self.reader)?;
//
//         for i in 0..archive.len() {
//             let mut file = archive.by_index(i)?;
//             let outpath = path.as_ref().join(file.enclosed_name().unwrap());
//
//             // {
//             //     let comment = file.comment();
//             //     if !comment.is_empty() {
//             //         println!("File {} comment: {}", i, comment);
//             //     }
//             // }
//
//             if (&*file.name()).ends_with('/') {
//                 // println!(
//                 //     "File {} extracted to \"{}\"",
//                 //     i,
//                 //     outpath.as_path().display()
//                 // );
//                 fs::create_dir_all(&outpath)?;
//             } else {
//                 // println!(
//                 //     "Extracting file {} to \"{}\" ({} bytes)",
//                 //     i,
//                 //     outpath.as_path().display(),
//                 //     file.size()
//                 // );
//                 if let Some(p) = outpath.parent() {
//                     if !p.exists() {
//                         fs::create_dir_all(&p)?;
//                     }
//                 }
//                 let mut outfile = fs::File::create(&outpath)?;
//                 io::copy(&mut file, &mut outfile)?;
//             }
//
//             // Get and Set permissions
//             #[cfg(unix)]
//             {
//                 use std::os::unix::fs::PermissionsExt;
//
//                 if let Some(mode) = file.unix_mode() {
//                     fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
//                 }
//             }
//         }
//         Ok(())
//     }
// }
