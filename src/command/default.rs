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
        set_symlink(&config.channel, "sb_penvman_cli", "sb-penvman")?;
        set_symlink(&config.channel, "sb_compiler_cli", "sb-compiler")?;
        set_symlink(&config.channel, "sb_linker_cli", "sb-linker")?;
        set_symlink(&config.channel, "sb_assembler_cli", "sb-assembler")?;
        set_symlink(&config.channel, "sb_emulator_cli", "sb-emulator-cli")?;
        set_symlink(&config.channel, "sb_emulator_tui", "sb-emulator-tui")?;
        set_symlink(&config.channel, "sb_builder_cli", "sb-builder")?;
        set_symlink(&config.channel, "sb_objdump_cli", "sb-objdump")?;

        Ok(())
    }
}

fn set_symlink(channel: &str, bin: &str, sym: &str) -> anyhow::Result<()> {
    let home_dir = env::var("HOME")?;
    let bin_path = format!("{}/.shinrabansyo/toolchains/{}/{}", home_dir, channel, bin);
    let ln_path = format!("{}/.shinrabansyo/bin/{}", home_dir, sym);

    StdCommand::new("ln")
        .arg("-sf")
        .arg(&bin_path)
        .arg(&ln_path)
        .output()?;

    Ok(())
}
