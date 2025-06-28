use std::env;
use std::fs;
use std::io::Write;
use std::thread;
use std::time::Duration;

use spinners::{Spinner, Spinners};

use crate::command::Command;
use crate::config::Config;
use crate::utils::repo::Repository;
use crate::CliOptions;

#[derive(Debug, Clone)]
pub struct Update {
    force: bool,
}

impl From<CliOptions> for Update {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Update { force } => Update { force },
            _ => unreachable!(),
        }
    }
}

impl Command for Update {
    fn run(self) -> anyhow::Result<()> {
        // 1. 設定ファイルの読み込み
        let config = Config::load()?;

        // 2. リンク配置用ディレクトリを作成
        let ln_dir = format!(
            "{}/.shinrabansyo/toolchains/{}",
            env::var("HOME")?,
            config.channel,
        );
        fs::create_dir_all(&ln_dir)?;

        // 3. 更新作業
        update_repo(&config.channel, self.force, "penv-manager")?;
        update_repo(&config.channel, self.force, "compiler")?;
        update_repo(&config.channel, self.force, "linker")?;
        update_repo(&config.channel, self.force, "assembler")?;
        update_repo(&config.channel, self.force, "emulator")?;
        update_repo(&config.channel, self.force, "builder")?;

        Ok(())
    }
}

fn update_repo(channel: &str, force: bool, repo_name: &str) -> anyhow::Result<()> {
    // 1. アニメーション開始
    let mut spinner = Spinner::new(
        Spinners::Dots,
        format!("Installing {:15}... ", repo_name),
    );
    let mut finish_spinner = |msg: &str| -> anyhow::Result<()> {
        spinner.stop();
        set_status(msg)?;
        println!("");
        Ok(())
    };
    thread::sleep(Duration::from_millis(100));

    // 2. リポジトリ取得
    set_status("checking updates")?;
    let mut repo = Repository::new(channel, repo_name)?;
    repo.sync_repo()?;
    if !force && !repo.check_updated()? {
        finish_spinner("Skipped")?;
        return Ok(());
    }

    // 3. ビルド
    set_status("building")?;
    repo.build()?;

    // 4. アニメーションの後処理
    let finish_msg = format!("Ok (version: {})", repo.version()?);
    finish_spinner(&finish_msg)?;

    Ok(())
}

fn set_status(stat: &str) -> anyhow::Result<()> {
    print!("{:20}", stat);
    std::io::stdout().flush()?;
    Ok(())
}
