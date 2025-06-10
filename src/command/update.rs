use std::env;
use std::fs;
use std::io::Write;
use std::process::Command as StdCommand;
use std::thread;
use std::time::Duration;

use spinners::{Spinner, Spinners};

use crate::command::Command;
use crate::config::Config;
use crate::utils::repo::Repository;
use crate::CliOptions;

#[derive(Debug, Clone)]
pub struct Update;

impl From<CliOptions> for Update {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Update => Update,
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
        update_repo(&config.channel, "penv-manager")?;
        update_repo(&config.channel, "compiler")?;
        update_repo(&config.channel, "linker")?;
        update_repo(&config.channel, "assembler")?;
        update_repo(&config.channel, "emulator")?;
        update_repo(&config.channel, "builder")?;

        Ok(())
    }
}

fn update_repo(channel: &str, repo_name: &str) -> anyhow::Result<()> {
    let home_dir = env::var("HOME")?;
    let ln_dir = format!("{}/.shinrabansyo/toolchains/{}", home_dir, channel);

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
    let repo = Repository::sync_repo(channel, repo_name)?;
    if !repo.check_updated()? {
        finish_spinner("Skipped")?;
        return Ok(());
    }

    // 3. コンパイル
    set_status("building")?;
    let bin_paths = repo.build()?;

    // 4. シンボリックリンクの配置
    for bin_path in bin_paths {
        let bin_name = bin_path.split("/").last().unwrap();
        let ln_path = format!("{}/{}", ln_dir, bin_name);

        StdCommand::new("ln")
            .arg("-sf")
            .arg(&bin_path)
            .arg(&ln_path)
            .output()?;
    }

    // 5. アニメーションの後処理
    let finish_msg = format!("Ok (version: {})", repo.version()?);
    finish_spinner(&finish_msg)?;

    Ok(())
}

fn set_status(stat: &str) -> anyhow::Result<()> {
    print!("{:20}", stat);
    std::io::stdout().flush()?;
    Ok(())
}
