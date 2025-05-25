use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub enum CliOptions {
    /// Initialize the environment
    #[bpaf(command)]
    Init,

    /// Update the all toolchains
    #[bpaf(command)]
    Update,

    /// Set the default channel
    #[bpaf(command)]
    Default {
        #[bpaf(positional)]
        channel: String,
    }
}

fn main() -> anyhow::Result<()> {
    let opts = cli_options().run();
    match opts {
        CliOptions::Init => todo!(),
        CliOptions::Update => todo!(),
        CliOptions::Default { .. } => todo!(),
    }
}
