use crate::config::Config;
use crate::shell::{bash, fish, zsh, Shell, ShellKind};
use clap::Clap;

#[derive(Debug, Clap, PartialEq, Eq)]
pub struct Env {
    #[clap(short, long)]
    use_on_cd: bool,

    /// Appends the snm path to the end of $PATH
    #[clap(short, long)]
    append: bool,

    #[clap(subcommand)]
    shell: ShellKind,
}

impl super::Command for Env {
    type InitResult = ();

    fn init(&self, config: Config) -> anyhow::Result<Self::InitResult> {
        let shell: &dyn Shell = match &self.shell {
            ShellKind::Bash => &bash::Bash,
            ShellKind::Zsh => &zsh::Zsh,
            ShellKind::Fish => &fish::Fish,
        };

        let path = super::bin_path(config.alias_default());

        println!("{}", shell.path(&path, self.append));

        println!("{}", shell.env_var("SNM_LOGLEVEL", &config.log_level));

        println!(
            "{}",
            shell.env_var("SNM_DIR", &config.snm_home().display().to_string())
        );

        println!(
            "{}",
            shell.env_var("SNM_NODE_DIST_MIRROR", &config.dist_mirror.to_string())
        );

        if self.use_on_cd {
            println!("{}", shell.use_on_cd());
        }

        Ok(())
    }
}
