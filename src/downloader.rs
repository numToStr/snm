use crate::fetcher::Release;
use crate::symlink::symlink_to;
use crate::url;
use crate::{archive::Archive, progress_bar::Bar};
use crate::{config::Config, url::Dist};
use colored::*;
use indicatif::HumanBytes;
use std::path::{Path, PathBuf};

pub struct Downloader<'a> {
    release: &'a Release,
    config: &'a Config,
    dist: Dist,
}

impl<'a> Downloader<'a> {
    pub fn new(release: &'a Release, config: &'a Config) -> Self {
        let dist = url::release(&config.dist_mirror, &release.version);

        Self {
            release,
            config,
            dist,
        }
    }

    pub fn download(&self) -> anyhow::Result<Vec<u8>> {
        let r = &self.release;
        let res = ureq::get(&self.dist.url).call()?;
        let len = res
            .header("Content-Length")
            .and_then(|x| x.parse::<u64>().ok());

        let size = match len {
            Some(l) => HumanBytes(l).to_string(),
            None => "unknown".into(),
        };

        println!("Version   : {}", r.version.to_string().bold());
        println!("Download  : {}", self.dist.url.bold());
        println!("Size      : {}", size.bold());

        println!();

        let buf = Bar::new(len).read_start(res.into_reader())?;

        println!();

        Ok(buf)
    }

    pub fn install(&self, buf: Vec<u8>) -> anyhow::Result<PathBuf> {
        let release_dir = self.config.release_dir();
        let dest = release_dir.join(&self.release.version.to_string());

        Archive::new(buf).extract_into(&release_dir)?;

        std::fs::rename(&release_dir.join(&self.dist.name), &dest)?;

        println!("Installed : {}", &dest.display().to_string().bold());

        Ok(dest)
    }

    pub fn alias_to_default(&self, dest: &Path) -> anyhow::Result<()> {
        symlink_to(&dest, &self.config.alias_default().as_path())?;

        println!();
        println!(
            "-- Using version {} --",
            self.release.version.to_string().bold()
        );

        Ok(())
    }
}

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
//             let buf = dwnld.download()?;
//             dwnld.install(buf)?
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
