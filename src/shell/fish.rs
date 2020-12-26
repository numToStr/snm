use clap::Clap;
use std::path::PathBuf;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Fish;

impl super::shell::Shell for Fish {
    fn path_env(&self, path: &PathBuf) -> String {
        format!("set -gx PATH {:?} $PATH;", path.to_str().unwrap_or(""))
    }

    fn env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }
}
