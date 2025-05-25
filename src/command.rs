mod init;

pub use init::Init;

use crate::CliOptions;

pub trait Command
where
    Self: From<CliOptions>,
{
    fn run(self) -> anyhow::Result<()>;
}
