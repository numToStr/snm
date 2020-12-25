use clap::Clap;
use std::path::PathBuf;

pub trait Shell {
    fn path_env(&self, path: &PathBuf) -> String;
    fn env_var(&self, name: &str, val: &str) -> String;
}

#[derive(Debug, Clap, PartialEq, Eq)]
pub enum ShellKind {
    /// Setup the zsh shell environment
    Zsh,

    /// Setup the fish shell environment
    Fish,
}
