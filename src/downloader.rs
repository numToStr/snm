use crate::config::Config;
use crate::fetcher::Release;
use crate::symlink::symlink_to;
use crate::url;
use crate::{archive::Archive, progress_bar::Bar};
use colored::*;
use indicatif::HumanBytes;
use std::path::PathBuf;
use ureq;

pub fn download(r: &Release, config: &Config) -> anyhow::Result<PathBuf> {
    let release_dir = &config.release_dir();
    let dest = release_dir.join(&r.version.to_string());

    if dest.exists() {
        anyhow::bail!(
            "Version {} is already exists locally",
            &r.version.to_string().bold()
        );
    }

    let dist = url::release(&config.dist_mirror, &r.version);
    let res = ureq::get(&dist.url).call()?;
    let len = res
        .header("Content-Length")
        .and_then(|x| x.parse::<u64>().ok());

    let size = match len {
        Some(l) => HumanBytes(l).to_string(),
        None => "unknown".into(),
    };

    println!("Installing  : {}", r.version.to_string().bold());
    println!("Downloading : {}", dist.url.bold());
    println!("Size        : {}", size.bold());

    let buf = Bar::new(len).read_start(res.into_reader())?;

    Archive::new(buf).extract_into(&release_dir)?;

    std::fs::rename(&release_dir.join(dist.name), &dest)?;

    println!("Installed  : {}", &dest.display().to_string().bold());

    // If we are only downloading then don't need to create a symlink to default
    if !config.download_only {
        println!("Alias      : {}", "default".bold());
        symlink_to(&dest, &config.alias_default())?;
    }

    Ok(dest)
}

// Conflicting with cross in CI
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::fetcher::Lts;
//     use crate::version::*;
//
//     #[test]
//     fn download_test() {
//         let config = Config::default();
//         let release = Release {
//             version: NodeVersion::parse("10.20.0").unwrap(),
//             lts: Lts::Yes("Dubnium".to_string()),
//         };
//         let dir = config.release_dir();
//         let download_path_expected = dir.join(release.version.to_string());
//         let download_path_result = download(&release, &config).unwrap();
//
//         assert_eq!(download_path_expected, download_path_result);
//
//         std::fs::remove_dir_all(dir).unwrap();
//         std::fs::remove_dir_all(config.alias_dir()).unwrap();
//     }
// }
