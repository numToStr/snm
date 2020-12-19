use crate::fetcher::Release;
use crate::url;
use crate::xtract::Xtract;
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
        println!("Size       : {}", &len);

        Xtract::new(res).extract_into("./");

        // path_str
        format!("Done {}", r.version)
    }
}

// installing : node-v14.15.3
// mkdir : /home/hello/n/n/versions/node/14.15.3
// fetch : https://nodejs.org/dist/v14.15.3/node-v14.15.3-linux-x64.tar.xz
// fetch : https://nodejs.org/download/release/v14.15.3/node-v14.15.3-linux-x64.tar.xz
// installed : v14.15.3 (with npm 6.14.9)
