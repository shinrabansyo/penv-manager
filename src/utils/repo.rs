use std::env;
use std::fs;
use std::process::Command as StdCommand;

use chrono::{DateTime, FixedOffset};
use git2::{BranchType, Repository as Git2Repository};

pub struct Repository<'a> {
    channel: &'a str,
    name: &'a str,
    git_repo: Git2Repository,
    git_branch: &'a str,
}

impl<'a> Repository<'a> {
    pub fn new(channel: &'a str, name: &'a str) -> anyhow::Result<Repository<'a>> {
        let home_dir = env::var("HOME")?;
        let repo_path = format!("{}/.shinrabansyo/repos/{}", home_dir, name);
        let repo_url = format!("https://github.com/shinrabansyo/{}", name);

        let git_repo = if !fs::exists(&repo_path)? {
            Git2Repository::clone(&repo_url, &repo_path)?
        } else {
            Git2Repository::open(&repo_path)?
        };

        Ok(Repository { channel, name, git_repo, git_branch: channel })
    }

    pub fn sync_repo(&mut self) -> anyhow::Result<()> {
        const GIT_NO_CREDENTIAL_OPT: &str = "credential.helper='!f() { cat > /dev/null; echo username=; echo password=; }; f'";

        let home_dir = env::var("HOME")?;
        let repo_path = format!("{}/.shinrabansyo/repos/{}", home_dir, self.name);
        let branch_channel = format!("origin/{}", self.git_branch);

        // 1. リポジトリの更新
        StdCommand::new("git")
            .arg("-c")
            .arg(GIT_NO_CREDENTIAL_OPT)
            .arg("fetch")
            .current_dir(&repo_path)
            .output()?;

        // 2. ビルド対象ブランチが存在するか確認
        let has_channel_branch = self.git_repo
            .branches(None)?
            .into_iter()
            .filter_map(Result::ok)
            .filter(|(_, branch_type)| branch_type == &BranchType::Remote)
            .find(|(branch, _)| branch.name().unwrap().unwrap() == branch_channel)
            .is_some();
        if !has_channel_branch {
            self.git_branch = "master";
            return self.sync_repo();
        }

        // 3. ビルド対象ブランチ選択
        StdCommand::new("git")
            .arg("merge")
            .arg(branch_channel)
            .arg(self.git_branch)
            .current_dir(&repo_path)
            .output()?;
        StdCommand::new("git")
            .arg("checkout")
            .arg(self.git_branch)
            .current_dir(&repo_path)
            .output()?;

        Ok(())
    }

    pub fn version(&self) -> anyhow::Result<String> {
        let head = self.git_repo.head()?.peel_to_commit()?;
        let head_datetime = DateTime::from_timestamp(head.time().seconds(), 0)
            .unwrap()
            .with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        Ok(head_datetime)
    }

    pub fn check_updated(&self) -> anyhow::Result<bool> {
        let home_dir = env::var("HOME")?;
        let repo_head_path = format!("{}/.shinrabansyo/repos/{}.head", home_dir, self.name);

        let old_head = fs::read_to_string(&repo_head_path).unwrap_or_default();
        let head = self.git_repo
            .head()?
            .peel_to_commit()?
            .id()
            .to_string();

        Ok(old_head != head)
    }

    pub fn build(&self) -> anyhow::Result<()> {
        let home_dir = env::var("HOME")?;
        let target_path = format!("{}/.shinrabansyo/repos/{}/target/release", home_dir, self.name);
        let repo_head_path = format!("{}/.shinrabansyo/repos/{}.head", home_dir, self.name);
        let ln_dir = format!("{}/.shinrabansyo/toolchains/{}", home_dir, self.channel);

        // 1. ビルド
        StdCommand::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(self.git_repo.path())
            .output()?;

        // 2. ビルド結果を収集
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
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        // 3. シンボリックリンクの配置
        for bin_path in bin_paths {
            let bin_name = bin_path.split("/").last().unwrap();
            let ln_path = format!("{}/{}", ln_dir, bin_name);

            StdCommand::new("ln")
                .arg("-sf")
                .arg(&bin_path)
                .arg(&ln_path)
                .output()?;
        }

        // 4. head ファイルの更新
        let head = self.git_repo
            .head()?
            .peel_to_commit()?
            .id()
            .to_string();
        fs::write(repo_head_path, &head)?;

        Ok(())
    }
}
