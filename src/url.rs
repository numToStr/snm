use crate::version::NodeVersion;

pub fn index(base_path: &str) -> String {
    format!("{}/index.json", base_path)
}

pub struct Dist {
    /// URL of the binary to be downloaded
    pub url: String,

    /// Name of the binary (without the extension)
    pub name: String,
}

#[cfg(unix)]
pub fn release(base_path: &str, v: &NodeVersion) -> Dist {
    use crate::sysinfo::{platform_arch, platform_name};

    let name = format!("node-{}-{}-{}", v, platform_name(), platform_arch());

    Dist {
        url: format!("{}/{}/{}.tar.xz", base_path, v, &name),
        name,
    }
}

#[cfg(windows)]
pub fn release(base_path: &str, v: &NodeVersion) -> Binary {
    use crate::sysinfo::platform_arch;

    let name = format!("node-{}-win-{}", v, platform_arch());

    Binary {
        url: format!("{}/{}/{}.zip", base_path, v, &name),
        name,
    }
}
