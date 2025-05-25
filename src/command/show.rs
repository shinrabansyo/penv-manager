use std::env;

use crate::command::Command;
use crate::config::Config;
use crate::CliOptions;

#[derive(Debug, Clone)]
pub struct Show;

impl From<CliOptions> for Show {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Show => Show,
            _ => unreachable!(),
        }
    }
}

impl Command for Show {
    fn run(self) -> anyhow::Result<()> {
        // 1. 設定ファイルの読み込み
        let config = Config::load()?;

        // 2. 設定値表示
        println!("Install Path : {}/.shinrabansyo", env::var("HOME")?);
        println!("Channel      : {}", config.channel);

        Ok(())
    }
}
