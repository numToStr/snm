use std::path::PathBuf;

pub trait Shell {
    fn path_env(&self, path: &PathBuf) -> String;
    fn env_var(&self, name: &str, val: &str) -> String;
}
