mod default;
mod init;
mod update;
mod show;

pub use default::Default;
pub use init::Init;
pub use update::Update;
pub use show::Show;

use crate::CliOptions;

pub trait Command
where
    Self: From<CliOptions>,
{
    fn run(self) -> anyhow::Result<()>;
}
