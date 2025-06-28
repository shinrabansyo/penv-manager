mod command;
mod config;
mod utils;

use bpaf::Bpaf;

use command::{Command, Default, Init, Update, Show};

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub enum CliOptions {
    /// Initialize the environment
    #[bpaf(command)]
    Init,

    /// Update the all toolchains
    #[bpaf(command)]
    Update {
        #[bpaf(short, long, switch)]
        force: bool,
    },

    /// Set the default channel
    #[bpaf(command)]
    Default {
        #[bpaf(positional)]
        channel: String,
    },

    /// Show the current configuration
    #[bpaf(command)]
    Show,
}

fn main() -> anyhow::Result<()> {
    let opts = cli_options().run();
    match opts {
        CliOptions::Init => Init::from(opts).run(),
        CliOptions::Update { .. } => Update::from(opts).run(),
        CliOptions::Default { .. } => Default::from(opts).run(),
        CliOptions::Show => Show::from(opts).run(),
    }
}
