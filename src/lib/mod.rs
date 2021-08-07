pub mod downloader;
pub mod fetcher;
pub mod linker;
pub mod loader;
pub mod shell;
pub mod sysinfo;
pub mod types;
pub mod version;

pub type SnmRes<T> = anyhow::Result<T>;

pub const MIRROR: &str = "https://nodejs.org/dist";
