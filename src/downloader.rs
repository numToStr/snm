use crate::fetcher::Release;
use crate::url;
use std::{fs::File, io::copy, path::Path};
use ureq;

pub struct Downloader;

impl Downloader {
    pub fn download(&self, r: &Release) -> String {
        let url = url::release(&r.version);

        let res = ureq::get(&url).call();
        let len = res
            .header("Content-Length")
            .and_then(|x| x.parse::<usize>().ok())
            .unwrap();

        println!("Installing : {}", &r.version);
        println!("Dowloading : {}", &url);
        println!("Size : {}", &len);

        let mut reader = res.into_reader();
        let path_str = format!("{}.tar.gz", r.version);
        let mut file = File::create(Path::new(path_str.as_str())).unwrap();

        copy(&mut reader, &mut file).unwrap();

        path_str
    }
}

// installing : node-v14.15.3
// mkdir : /home/hello/n/n/versions/node/14.15.3
// fetch : https://nodejs.org/dist/v14.15.3/node-v14.15.3-linux-x64.tar.xz
// fetch : https://nodejs.org/download/release/v14.15.3/node-v14.15.3-linux-x64.tar.xz
// installed : v14.15.3 (with npm 6.14.9)
