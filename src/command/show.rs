use std::env;

use spinners::{Spinner, Spinners};

use crate::command::Command;
use crate::config::Config;
use crate::utils::repo::Repository;
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

        // 2. 各リポジトリのバージョンを取得
        let mut spinner = Spinner::new(Spinners::Dots, "Loading ...".into());
        let repo_versions = vec![
            ("penv-manager", repo_version(&config, "penv-manager")?),
            ("compiler", repo_version(&config, "compiler")?),
            ("linker", repo_version(&config, "linker")?),
            ("assembler", repo_version(&config, "assembler")?),
            ("emulator", repo_version(&config, "emulator")?),
            ("builder", repo_version(&config, "builder")?),
        ];
        spinner.stop();

        // 3. 各値表示
        println!("\rInstall Path : {}/.shinrabansyo", env::var("HOME")?);
        println!("Channel      : {}", config.channel);
        println!("Repository   :");
        for (name, version) in repo_versions {
            println!("    * {:12} : {}", name, version);
        }

        Ok(())
    }
}

fn repo_version(config: &Config, repo_name: &str) -> anyhow::Result<String> {
    Repository::new(&config.channel, repo_name)?.version()
}
