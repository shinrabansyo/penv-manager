mod init;
mod update;

pub use init::Init;
pub use update::Update;

use crate::CliOptions;

pub trait Command
where
    Self: From<CliOptions>,
{
    fn run(self) -> anyhow::Result<()>;
}
