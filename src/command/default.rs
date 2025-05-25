use std::env;
use std::process::Command as StdCommand;

use crate::command::Command;
use crate::config::Config;
use crate::CliOptions;

#[derive(Debug, Clone)]
pub struct Default {
    channel: String,
}

impl From<CliOptions> for Default {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Default { channel } => Default { channel },
            _ => unreachable!(),
        }
    }
}

impl Command for Default {
    fn run(self) -> anyhow::Result<()> {
        // 1. 設定ファイルの読み込み
        let mut config = Config::load()?;

        // 2. 更新
        config.channel = self.channel;
        config.store()?;

        // 3. シンボリックリンクを更新
        set_symlink("sb_compiler", &config.channel)?;
        set_symlink("sb_linker", &config.channel)?;
        set_symlink("sb_assembler", &config.channel)?;
        set_symlink("sb_builder", &config.channel)?;
        set_symlink("sb_debugger", &config.channel)?;

        Ok(())
    }
}

fn set_symlink(bin: &str, channel: &str) -> anyhow::Result<()> {
    let home_dir = env::var("HOME")?;
    let bin_path = format!("{}/.shinrabansyo/toolchains/{}/{}", home_dir, channel, bin);
    let ln_path = format!("{}/.shinrabansyo/bin/{}", home_dir, bin);

    StdCommand::new("ln")
        .arg("-sf")
        .arg(&bin_path)
        .arg(&ln_path)
        .output()?;

    Ok(())
}
