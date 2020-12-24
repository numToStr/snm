use crate::version::NodeVersion;

pub fn base_dist<'a>() -> &'a str {
    "https://nodejs.org/download/release"
}

pub fn index() -> String {
    format!("{}/index.json", base_dist())
}

pub struct Binary {
    /// URL of the binary to be downloaded
    pub url: String,

    /// Name of the binary (without the extension)
    pub name: String,
}

#[cfg(unix)]
pub fn release(v: &NodeVersion) -> Binary {
    use crate::sysinfo::{platform_arch, platform_name};

    let name = format!("node-{}-{}-{}", v, platform_name(), platform_arch());

    Binary {
        url: format!("{}/{}/{}.tar.xz", base_dist(), v, &name),
        name,
    }
}

#[cfg(windows)]
pub fn release(v: &str) -> Binary {
    use crate::sysinfo::platform_arch;

    let name = format!("node-{}-win-{}", v, platform_arch());

    Binary {
        url: format!("{}/{}/{}.zip", base_dist(), v, &name),
        name,
    }
}
