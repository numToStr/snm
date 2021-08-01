use std::{
    io::Read,
    path::{Path, PathBuf},
};

use indicatif::HumanBytes;
use tempfile::Builder;
use url::Url;

use super::{version::dist_version::DistVersion, SnmRes};

#[derive(Debug)]
struct Dist(pub String);

impl Dist {
    fn new(mirror: &Url, version: &DistVersion) -> Self {
        use crate::sysinfo::{platform_arch, platform_name};

        let version: String = version.into();

        Dist(format!(
            "{}/v{ver}/node-v{ver}-{}-{}.tar.xz",
            mirror,
            platform_name(),
            platform_arch(),
            ver = version
        ))
    }
}

impl AsRef<str> for Dist {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct Downloader2<'a> {
    version: &'a DistVersion,
    dist: Dist,
}

impl<'a> Downloader2<'a> {
    pub fn new(mirror: &'a Url, version: &'a DistVersion) -> Self {
        let dist = Dist::new(mirror, version);

        Self { version, dist }
    }

    fn extract_to(&self, source: impl Read + Send, dest: &Path) -> SnmRes<()> {
        let xz_stream = xz2::read::XzDecoder::new(source);
        let mut tar_stream = tar::Archive::new(xz_stream);
        let entries = tar_stream.entries()?;

        for entry in entries {
            let mut entry = entry?;
            let entry_path: PathBuf = entry.path()?.iter().skip(1).collect();
            let dest = dest.join(entry_path);

            entry.unpack(dest)?;
        }

        Ok(())
    }

    pub fn download(&self, release_dir: &Path) -> SnmRes<PathBuf> {
        let version: String = self.version.into();

        let dest = release_dir.join(&version);

        if dest.exists() {
            anyhow::bail!("Version {} is already exists locally", version);
        }

        let tmp_dir = Builder::new().tempdir_in(release_dir)?;

        let resp = ureq::get(&self.dist.0).call()?;

        let len = resp
            .header("Content-Length")
            .and_then(|x| x.parse::<u64>().ok());

        let size = match len {
            Some(l) => HumanBytes(l).to_string(),
            None => "unknown".into(),
        };

        println!("Version   : {}", version);
        println!("Release   : {}", self.dist.as_ref());
        println!("Size      : {}", size);

        self.extract_to(resp.into_reader(), tmp_dir.as_ref())?;

        std::fs::rename(tmp_dir.as_ref(), &dest)?;

        println!();
        println!("Installed : {}", dest.display());

        Ok(dest)
    }
}
