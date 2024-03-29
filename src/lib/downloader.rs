use crate::{
    loader::{Bar, Spinner},
    sysinfo::{platform_arch, platform_name},
    types::{DownloadDir, ReleaseDir},
};
use console::style;
use indicatif::HumanBytes;
use std::{io::Read, path::PathBuf};
use url::Url;

use super::{version::DistVersion, SnmRes};

#[derive(Debug)]
struct Dist(String);

impl Dist {
    fn new(mirror: &Url, version: &DistVersion) -> Self {
        #[cfg(unix)]
        let extension = "tar.xz";

        #[cfg(windows)]
        let extension = "zip";

        Dist(format!(
            "{}/v{version}/node-v{version}-{}-{}.{}",
            mirror,
            platform_name(),
            platform_arch(),
            extension,
            version = version
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

    #[cfg(unix)]
    fn extract_to<S: Read + Send>(
        &self,
        source: &mut S,
        r_dir: &ReleaseDir,
        d_dir: &DownloadDir,
    ) -> SnmRes<()> {
        let tmp_dir = tempfile::Builder::new().tempdir_in(d_dir.as_ref())?;

        let xz_stream = xz2::read::XzDecoder::new(source);
        let mut tar_stream = tar::Archive::new(xz_stream);
        let entries = tar_stream.entries()?;

        for entry in entries {
            let mut entry = entry?;

            // Stripping the first path segment which is usually the name of the file
            let entry_path: PathBuf = entry.path()?.iter().skip(1).collect();

            let tmp_dir = tmp_dir.as_ref().join(entry_path);

            entry.unpack(tmp_dir)?;
        }

        let install_dir = r_dir.as_ref().join(self.version.to_string());

        std::fs::rename(&tmp_dir, &install_dir)?;

        Ok(())
    }

    #[cfg(windows)]
    fn extract_to<S: Read + Send>(
        &self,
        source: &mut S,
        r_dir: &ReleaseDir,
        d_dir: &DownloadDir,
    ) -> SnmRes<()> {
        use std::{fs, io};

        let mut tmp_file = tempfile::Builder::new().tempfile_in(d_dir.as_ref())?;

        io::copy(source, &mut tmp_file)?;

        let mut archive = zip::read::ZipArchive::new(&tmp_file)?;

        let install_dir = r_dir.as_ref().join(self.version.to_string());

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            let outpath = {
                let f = match file.enclosed_name() {
                    // Stripping the first path segment which is usually the name of the file
                    Some(path) => path.iter().skip(1).collect::<PathBuf>(),
                    None => continue,
                };

                install_dir.join(f)
            };

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File {} comment: {}", i, comment);
                }
            }

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p)?;
                    }
                }

                let mut outfile = fs::File::create(&outpath)?;

                io::copy(&mut file, &mut outfile)?;
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }

        Ok(())
    }

    pub fn download(&self, r_dir: &ReleaseDir, d_dir: &DownloadDir) -> SnmRes<ReleaseDir> {
        let version: String = self.version.to_string();

        let dest = r_dir.join(&version);

        if dest.as_ref().exists() {
            anyhow::bail!("Version {} already exists locally", style(version).bold());
        }

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
        println!();

        if let Some(l) = len {
            let bar = Bar::new(l);

            self.extract_to(&mut bar.take_reader(resp.into_reader()), r_dir, d_dir)?;

            bar.finish();
        } else {
            let spinner = Spinner::new("Installing...");

            self.extract_to(&mut resp.into_reader(), r_dir, d_dir)?;

            spinner.finish()
        }

        println!("Installed : {}", style(dest.as_ref().display()).bold());

        Ok(dest)
    }
}
