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
        update_repo("penv-manager", "sb-penvman", &config.channel)?;
        update_repo("compiler", "sb-compiler", &config.channel)?;
        update_repo("linker", "sb-linker", &config.channel)?;
        update_repo("assembler", "sb-assembler", &config.channel)?;
        update_repo("builder", "sb-builder", &config.channel)?;
        update_repo("debugger", "sb-debugger", &config.channel)?;

        Ok(())
    }
}

fn update_repo(repo: &str, bin: &str, channel: &str) -> anyhow::Result<()> {
    const GIT_NO_CREDENTIAL_OPT: &str = "credential.helper='!f() { cat > /dev/null; echo username=; echo password=; }; f'";

    let home_dir = env::var("HOME")?;
    let repo_par_path = format!("{}/.shinrabansyo/repos", home_dir);
    let repo_path = format!("{}/.shinrabansyo/repos/{}", home_dir, repo);
    let repo_url = format!("https://github.com/shinrabansyo/{}", repo);
    let target_path = format!("{}/.shinrabansyo/repos/{}/target/release", home_dir, repo);
    let ln_path = format!("{}/.shinrabansyo/toolchains/{}/{}", home_dir, channel, bin);

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
    let bin_path = String::from_utf8(bin_path)?
        .split("\n")
        .into_iter()
        .filter(|s| s.contains("sb_"))
        .next()
        .unwrap()
        .to_string();

    // 6. シンボリックリンクの配置
    StdCommand::new("ln")
        .arg("-sf")
        .arg(&bin_path.trim())
        .arg(&ln_path)
        .output()?;

    // 7. アニメーションの後処理
    spinner.stop();
    println!("Ok");

    Ok(())
}
