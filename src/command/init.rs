use std::env;
use std::fs;

use crate::command::{Command, CliOptions};

#[derive(Debug, Clone)]
pub struct Init;

impl From<CliOptions> for Init {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Init => Init,
            _ => unreachable!(),
        }
    }
}

impl Command for Init {
    fn run(self) -> anyhow::Result<()> {
        let home_dir = env::var("HOME")?;

        // 1. ディレクトリ作成
        fs::create_dir_all(format!("{}/.shinrabansyo/bin", home_dir))?;
        fs::create_dir_all(format!("{}/.shinrabansyo/repos", home_dir))?;
        fs::create_dir_all(format!("{}/.shinrabansyo/toolchains", home_dir))?;

        // 2. 設定ファイル初期化
        fs::write(
            format!("{}/.shinrabansyo/config.toml", home_dir),
            "channel = \"master\"\n",
        )?;

        // 3. env ファイル初期化
        fs::write(
            format!("{}/.shinrabansyo/env", home_dir),
            "\n",
        )?;

        Ok(())
    }
}
