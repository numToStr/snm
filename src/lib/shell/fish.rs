use super::Shell;
use std::path::Path;

#[derive(Debug)]
pub struct Fish;

impl Shell for Fish {
    fn path(&self, path: &Path, append: bool) -> String {
        if append {
            return format!("set -gx PATH $PATH {:?};", path.display());
        }

        format!("set -gx PATH {:?} $PATH;", path.display())
    }

    fn env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }

    fn use_on_cd(&self) -> &'static str {
        indoc::indoc!(
            r#"
                function _snm_autoload_hook --on-variable PWD --description 'Change Node version on directory change'
                    status --is-command-substitution; and return
                    if test -f .node-version -o -f .nvmrc -o -f package.json
                        snm use
                    end
                end
            "#
        )
    }
}
