use crate::config::Config;
use crate::lib::{
    shell::{bash, fish, pwsh, zsh, Shell, ShellKind},
    SnmRes,
};
use clap::Clap;

#[derive(Debug, Clap)]
pub struct Env {
    /// Add the shell script to run `snm use` on directory change
    #[clap(short, long)]
    use_on_cd: bool,

    /// Appends the snm path to the end of $PATH
    #[clap(short, long)]
    append: bool,

    /// Type of shell you want to configure
    #[clap(subcommand)]
    shell: ShellKind,
}

impl super::Command for Env {
    fn init(self, config: Config) -> SnmRes<()> {
        let shell: &dyn Shell = match &self.shell {
            ShellKind::Bash => &bash::Bash,
            ShellKind::Zsh => &zsh::Zsh,
            ShellKind::Fish => &fish::Fish,
            ShellKind::Pwsh => &pwsh::Pwsh,
        };

        let path = config.bin_path(config.alias_default());

        println!("{}", shell.path(&path, self.append));

        // println!("{}", shell.env_var("SNM_LOGLEVEL", &config.log_level));

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
