use super::Shell;
use std::path::Path;

#[derive(Debug)]
pub struct Pwsh;

impl Shell for Pwsh {
    fn path(&self, path: &Path, append: bool) -> String {
        let path_env = std::env::var_os("PATH").unwrap_or_default();
        let mut split_paths: Vec<_> = std::env::split_paths(&path_env).collect();

        if append {
            split_paths.push(path.to_path_buf());
        } else {
            split_paths.insert(0, path.to_path_buf());
        }

        let new_path = std::env::join_paths(split_paths).expect("Can't join paths");
        self.env_var("PATH", new_path.to_str().expect("Can't read PATH"))
    }

    fn env_var(&self, name: &str, val: &str) -> String {
        format!(r#"$env:{} = "{}""#, name, val)
    }

    fn use_on_cd(&self) -> &'static str {
        indoc::indoc!(
            r#"
                function Set-LocationWithsnm { param($path); Set-Location $path; If ((Test-Path .nvmrc) -Or (Test-Path .node-version) -Or (Test-Path package.json)) { & snm use } }
                Set-Alias cd_with_snm Set-LocationWithsnm -Force
                Remove-Item alias:\cd
                New-Alias cd Set-LocationWithsnm
            "#
        )
    }
}
