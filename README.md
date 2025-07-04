# SB Penv-Manager

## Install

```sh
curl --proto '=https' --tlsv1.2 -sSf https://shinrabansyo.github.io/penv-manager/ | sh
```

## Tools

以下のツールがインストールされます

- `sb-penvman` : 本ツール
- `sb-compiler` : コンパイラ ([GitHub Repo](https://github.com/shinrabansyo/compiler))
- `sb-linker` : リンカ ([GitHub Repo](https://github.com/shinrabansyo/linker))
- `sb-assembler` : アセンブラ ([GitHub Repo](https://github.com/shinrabansyo/assembler))
- `sb-emulator-cli` : エミュレータ (CLI) ([GitHub Repo](https://github.com/shinrabansyo/emulator))
- `sb-emulator-tui` : エミュレータ (TUI) ([GitHub Repo](https://github.com/shinrabansyo/emulator))
- `sb-builder` : ビルドシステム ([GitHub Repo](https://github.com/shinrabansyo/builder))

## Command

```sh
$ sb-penvman COMMAND
```

- `init` : 初期セットアップ
- `update` : 各種ツールの更新
- `default` : デフォルトのチャネルを設定
- `show` : 現在の設定内容を表示

## Note

- Linux 環境での実行を前提としています
- `git`，`cargo(nightly)` が必要です
- 各種ファイルは `~/.shinrabansyo` にインストールされます
