use crate::downloader::Downloader;
use crate::fetcher::Releases;
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Lts;

impl Lts {
    pub fn init(&self) -> String {
        let mut release = Releases::fetch();

        let r = release.lts().unwrap();

        Downloader.download(&r)
    }
}
