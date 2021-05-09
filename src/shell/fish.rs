use clap::Clap;
use std::path::Path;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Fish;

impl super::shell::Shell for Fish {
    fn path(&self, path: &Path, append: bool) -> String {
        if append {
            return format!("set -gx PATH $PATH {:?};", path.display());
        }

        format!("set -gx PATH {:?} $PATH;", path.display())
    }

    fn env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }

    fn use_on_cd(&self) -> String {
        indoc::indoc!(
            r#"
                function _fnm_autoload_hook --on-variable PWD --description 'Change Node version on directory change'
                    status --is-command-substitution; and return
                    if test -f .node-version -o -f .nvmrc
                        fnm use
                    end
                end
            "#
        )
        .into()
    }
}
