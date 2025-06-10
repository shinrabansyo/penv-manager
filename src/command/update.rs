use std::env;
use std::fs;
use std::io::Write;
use std::process::Command as StdCommand;
use std::thread;
use std::time::Duration;

use chrono::{DateTime, FixedOffset};
use git2::{BranchType, Repository};
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
    let home_dir = env::var("HOME")?;
    let repo_head_path = format!("{}/.shinrabansyo/repos/{}.head", home_dir, repo);
    let repo_path = format!("{}/.shinrabansyo/repos/{}", home_dir, repo);
    let target_path = format!("{}/.shinrabansyo/repos/{}/target/release", home_dir, repo);
    let ln_dir = format!("{}/.shinrabansyo/toolchains/{}", home_dir, channel);

    // 1. アニメーション開始
    let mut spinner = Spinner::new(
        Spinners::Dots,
        format!("Installing {:15}... ", repo),
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
    let git_repo = sync_repo(repo, channel)?;

    // 3. 更新の有無を確認
    let old_head = fs::read_to_string(&repo_head_path).unwrap_or_default();
    let head_commit = git_repo.head()?.peel_to_commit()?;
    if old_head == head_commit.id().to_string() {
        finish_spinner("Skipped")?;
        return Ok(());
    }

    // 4. コンパイル
    set_status("building")?;
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

    // 7. head ファイルの更新
    fs::write(&repo_head_path, head_commit.id().to_string())?;

    // 8. アニメーションの後処理
    let new_head_datetime = DateTime::from_timestamp(head_commit.time().seconds(), 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())
        .format("%Y-%m-%d %H:%M:%S");
    let finish_msg = format!("Ok (version: {})", new_head_datetime);
    finish_spinner(&finish_msg)?;

    Ok(())
}

fn sync_repo(repo: &str, channel: &str) -> anyhow::Result<Repository> {
    const GIT_NO_CREDENTIAL_OPT: &str = "credential.helper='!f() { cat > /dev/null; echo username=; echo password=; }; f'";

    let home_dir = env::var("HOME")?;
    let repo_path = format!("{}/.shinrabansyo/repos/{}", home_dir, repo);
    let repo_url = format!("https://github.com/shinrabansyo/{}", repo);
    let branch_channel = format!("origin/{}", channel);

    // 1. リポジトリ取得
    let git_repo = if !fs::exists(&repo_path)? {
        Repository::clone(&repo_url, &repo_path)?
    } else {
        Repository::open(&repo_path)?
    };

    // 2. リポジトリの更新
    StdCommand::new("git")
        .arg("-c")
        .arg(GIT_NO_CREDENTIAL_OPT)
        .arg("fetch")
        .current_dir(&repo_path)
        .output()?;

    // 3. ビルド対象ブランチが存在するか確認
    let has_channel_branch = git_repo
        .branches(None)?
        .into_iter()
        .filter_map(Result::ok)
        .filter(|(_, branch_type)| branch_type == &BranchType::Remote)
        .find(|(branch, _)| branch.name().unwrap().unwrap() == branch_channel)
        .is_some();
    if !has_channel_branch {
        return sync_repo(repo, "master");
    }

    // 4. ビルド対象ブランチ選択
    StdCommand::new("git")
        .arg("merge")
        .arg(branch_channel)
        .arg(channel)
        .current_dir(&repo_path)
        .output()?;
    StdCommand::new("git")
        .arg("checkout")
        .arg(channel)
        .current_dir(&repo_path)
        .output()?;

    Ok(git_repo)
}

fn set_status(stat: &str) -> anyhow::Result<()> {
    print!("{:20}", stat);
    std::io::stdout().flush()?;
    Ok(())
}
