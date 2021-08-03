use std::{
    io::Read,
    path::{Path, PathBuf},
};

use console::style;
use indicatif::HumanBytes;
use tempfile::Builder;
use url::Url;

use super::{version::dist_version::DistVersion, SnmRes};

#[derive(Debug)]
struct Dist(pub String);

impl Dist {
    fn new(mirror: &Url, version: &DistVersion) -> Self {
        use crate::sysinfo::{platform_arch, platform_name};

        // FIXME: windows support
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

pub struct Downloader<'a> {
    version: &'a DistVersion,
    dist: Dist,
}

impl<'a> Downloader<'a> {
    pub fn new(mirror: &'a Url, version: &'a DistVersion) -> Self {
        let dist = Dist::new(mirror, version);

        Self { version, dist }
    }

    // FIXME: windows support
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
        let version: String = self.version.to_string();

        let dest = release_dir.join(&version);

        if dest.exists() {
            anyhow::bail!("Version {} already exists locally", style(version).bold());
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

        println!("Version   : {}", style(version).bold());
        println!("Release   : {}", style(self.dist.as_ref()).bold());
        println!("Size      : {}", style(size).bold());

        self.extract_to(resp.into_reader(), tmp_dir.as_ref())?;

        std::fs::rename(tmp_dir.as_ref(), &dest)?;

        println!();
        println!("Installed : {}", style(dest.display()).bold());

        Ok(dest)
    }
}
