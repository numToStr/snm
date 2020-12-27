use std::path::PathBuf;

#[derive(Debug)]
pub struct Bash;

impl super::shell::Shell for Bash {
    fn path(&self, path: &PathBuf) -> String {
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
