pub fn base_dist<'a>() -> &'a str {
    "https://nodejs.org/download/release"
}

pub fn index() -> String {
    format!("{}/index.json", base_dist())
}

pub fn release(v: &str) -> String {
    // FIXME: different executable for
    // 1. os ie. mac, win, linux
    // 2. architecture i.e x86, x64, arm64
    format!("{}/{}/node-{}-linux-x64.tar.xz", base_dist(), v, v)
}
