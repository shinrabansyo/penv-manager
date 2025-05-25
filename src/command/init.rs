use std::env;
use std::fs;

use crate::command::{Command, Update, Default};
use crate::CliOptions;

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
            "channel = \"develop\"\n",
        )?;

        // 3. env ファイル初期化
        fs::write(
            format!("{}/.shinrabansyo/env", home_dir),
            r#"case ":${PATH}:" in *:"$HOME/.shinrabansyo/bin":*) ;; *) export PATH="$HOME/.shinrabansyo/bin:$PATH" ;; esac"#,
        )?;

        // 4. 各種ツールチェインの更新
        Update::from(CliOptions::Update).run()?;
        Default::from(CliOptions::Default { channel: "develop".to_string() }).run()?;

        // 5. 完了メッセージ
        println!(r#"+-----------------------------------------------------------------------------------------------+"#);
        println!(r#"| Shinrabansyo has been initialized.                                                            |"#);
        println!(r#"| Please add the following line to your shell configuration file (e.g., ~/.bashrc, ~/.zshrc):   |"#);
        println!(r#"|    source "$HOME/.shinrabansyo/env"                                                           |"#);
        println!(r#"| Then, restart your shell to apply the changes.                                                |"#);
        println!(r#"+-----------------------------------------------------------------------------------------------+"#);

        Ok(())
    }
}
