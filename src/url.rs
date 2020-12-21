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

pub fn release(v: &str) -> Binary {
    // FIXME: different executable for
    // 1. os ie. mac, win, linux
    // 2. architecture i.e x86, x64, arm64

    let name = format!("node-{}-linux-x64", v);

    Binary {
        url: format!("{}/{}/{}.tar.xz", base_dist(), v, &name),
        name,
    }
}
