// use std::io::{copy, Read};
use std::path::Path;
use tar::Archive;
// use tempfile::tempfile;
use ureq::Response;
use xz2::read::XzDecoder;
// use zip::read::ZipArchive;

// pub struct Zip {
//     res: Response,
// }
//
// impl Zip {
//     pub fn new(res: Response) -> Self {
//         Self { res }
//     }
//
//     pub fn extract_into() {}
// }

// pub fn from_zip(res: Response) {
//     let mut reader = res.into_reader();
//     let mut t_file = tempfile().unwrap();
//     copy(&mut reader, &mut t_file).unwrap();
//     let mut zip = ZipArchive::new(t_file).unwrap();
//
//     for i in 0..zip.len() {
//         let file = zip.by_index(i).unwrap();
//         println!("Filename: {}", file.name());
//         let first_byte = file.bytes().next().unwrap();
//         println!("{:#?}", first_byte);
//     }
// }

pub struct Xtract {
    res: Response,
}

impl Xtract {
    pub fn new(res: Response) -> Self {
        Xtract { res }
    }

    #[cfg(target_family = "unix")]
    pub fn extract_into<P: AsRef<Path>>(self, path: P) {
        let xz_stream = XzDecoder::new(self.res.into_reader());
        let mut archive = Archive::new(xz_stream);
        archive.unpack(path).unwrap();
    }

    #[cfg(target_family = "windows")]
    pub fn extract_into(self, path: &str) {}
}
