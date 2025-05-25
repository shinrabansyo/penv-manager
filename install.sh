# 1. ディレクトリ準備
mkdir -p ~/.shinrabansyo/repos
cd ~/.shinrabansyo/repos

# 2. リポジトリのクローン
git -c credential.helper='!f() { cat > /dev/null; echo username=; echo password=; }; f' clone https://github.com/shinrabansyo/penv-manager
cd penv-manager

# 3. penv-manager を実行
cargo run -- init
