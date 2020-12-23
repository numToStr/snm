use clap::Clap;
use std::path::PathBuf;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Zsh;

impl super::shell::Shell for Zsh {
    fn env_var(&self, name: &str, val: &str) -> String {
        format!("export {}={:?};", name, val)
    }

    fn path_env(&self, path: &PathBuf) -> String {
        format!("export PATH={:?}:$PATH;", path.to_str().unwrap())
    }
}
