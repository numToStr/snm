use clap::Clap;
use std::path::Path;

pub trait Shell {
    fn path(&self, path: &Path, append: bool) -> String;
    fn env_var(&self, name: &str, val: &str) -> String;
    fn use_on_cd(&self) -> String;
}

#[derive(Debug, Clap, PartialEq, Eq)]
pub enum ShellKind {
    /// Setup the bash shell environment
    Bash,

    /// Setup the zsh shell environment
    Zsh,

    /// Setup the fish shell environment
    Fish,

    /// Setup the Windows Powershell environment
    Pwsh,
}
