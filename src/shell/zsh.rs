use super::Shell;
use std::path::Path;

#[derive(Debug)]
pub struct Zsh;

impl Shell for Zsh {
    fn path(&self, path: &Path, append: bool) -> String {
        if append {
            return format!("export PATH=$PATH:{:?};", path.display());
        }

        format!("export PATH={:?}:$PATH;", path.display())
    }

    fn env_var(&self, name: &str, val: &str) -> String {
        format!("export {}={:?};", name, val)
    }

    fn use_on_cd(&self) -> String {
        indoc::indoc!(
            r#"
                autoload -U add-zsh-hook
                _fnm_autoload_hook () {
                    if [[ -f .node-version || -f .nvmrc ]]; then
                        fnm use
                    fi
                }
                add-zsh-hook -Uz chpwd _fnm_autoload_hook
            "#
        )
        .into()
    }
}
