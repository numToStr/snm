use crate::version::NodeVersion;
use url::Url;

pub fn index(base_url: &Url) -> String {
    format!("{}/index.json", base_url)
}

pub struct Dist {
    /// URL of the binary to be downloaded
    pub url: String,

    /// Name of the binary (without the extension)
    pub name: String,
}

#[cfg(unix)]
pub fn release(base_url: &Url, version: &NodeVersion) -> Dist {
    use crate::sysinfo::{platform_arch, platform_name};

    let name = format!("node-{}-{}-{}", version, platform_name(), platform_arch());

    Dist {
        url: format!("{}/{}/{}.tar.xz", base_url, version, &name),
        name,
    }
}

#[cfg(windows)]
pub fn release(base_url: &Url, version: &NodeVersion) -> Dist {
    use crate::sysinfo::platform_arch;

    let name = format!("node-{}-win-{}", version, platform_arch());

    Dist {
        url: format!("{}/{}/{}.zip", base_url, version, &name),
        name,
    }
}
