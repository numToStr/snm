pub mod downloader;
pub mod fetcher;
pub mod linker;
pub mod shell;
pub mod sysinfo;
pub mod version;

pub type SnmRes<T> = anyhow::Result<T>;
