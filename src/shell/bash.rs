use super::Shell;
use std::path::Path;

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn path(&self, path: &Path, append: bool) -> String {
        if append {
            return format!("export PATH=$PATH:{:?};", path.display());
        }

        format!("export PATH={:?}:$PATH", path.display())
    }

    fn env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
    }

    fn use_on_cd(&self) -> String {
        indoc::indoc!(
            r#"
                __fnm_use_if_file_found() {
                    if [[ -f .node-version || -f .nvmrc ]]; then
                        fnm use
                    fi
                }
                __fnmcd() {
                    \cd "$@" || return $?
                    __fnm_use_if_file_found
                }
                alias cd=__fnmcd
                __fnm_use_if_file_found
            "#
        )
        .into()
    }
}
