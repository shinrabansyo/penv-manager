use std::env;
use std::fs;
use std::process::Command as StdCommand;

use spinners::{Spinner, Spinners};

use crate::command::Command;
use crate::config::Config;
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

fn update_repo(channel: &str, repo: &str) -> anyhow::Result<()> {
    const GIT_NO_CREDENTIAL_OPT: &str = "credential.helper='!f() { cat > /dev/null; echo username=; echo password=; }; f'";

    let home_dir = env::var("HOME")?;
    let repo_par_path = format!("{}/.shinrabansyo/repos", home_dir);
    let repo_path = format!("{}/.shinrabansyo/repos/{}", home_dir, repo);
    let repo_url = format!("https://github.com/shinrabansyo/{}", repo);
    let target_path = format!("{}/.shinrabansyo/repos/{}/target/release", home_dir, repo);
    let ln_dir = format!("{}/.shinrabansyo/toolchains/{}", home_dir, channel);

    // 1. アニメーション開始
    let mut spinner = Spinner::new(
        Spinners::Dots,
        format!("Installing {:15}... ", repo),
    );

    // 2. リポジトリのクローン
    if !fs::exists(&repo_path)? {
        StdCommand::new("git")
            .arg("-c")
            .arg(GIT_NO_CREDENTIAL_OPT)
            .arg("clone")
            .arg(&repo_url)
            .current_dir(&repo_par_path)
            .output()?;
    }

    // 3. リポジトリの更新
    StdCommand::new("git")
        .arg("-c")
        .arg(GIT_NO_CREDENTIAL_OPT)
        .arg("pull")
        .arg("origin")
        .arg(format!("{}:{}", channel, channel))
        .current_dir(&repo_path)
        .output()?;
    StdCommand::new("git")
        .arg("checkout")
        .arg(channel)
        .current_dir(&repo_path)
        .output()?;

    // 4. コンパイル
    StdCommand::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(&repo_path)
        .output()?;

    // 5. コンパイル結果のパスを取得
    let bin_path = StdCommand::new("find")
        .arg(&target_path)
        .arg("-maxdepth")
        .arg("1")
        .arg("-type")
        .arg("f")
        .arg("-executable")
        .output()?
        .stdout;
    let bin_paths = String::from_utf8(bin_path)?;
    let bin_paths = bin_paths
        .split("\n")
        .into_iter()
        .filter(|s| s.contains("sb_") && !s.contains(".so"))
        .map(|s| s.trim());

    // 6. シンボリックリンクの配置
    for bin_path in bin_paths {
        let bin_name = bin_path.split("/").last().unwrap();
        let ln_path = format!("{}/{}", ln_dir, bin_name);

        StdCommand::new("ln")
            .arg("-sf")
            .arg(&bin_path)
            .arg(&ln_path)
            .output()?;
    }

    // 7. アニメーションの後処理
    spinner.stop();
    println!("Ok");

    Ok(())
}
