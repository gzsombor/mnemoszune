use structopt::StructOpt;

mod configuration;
use configuration::{CmdLine, ConfigFile};

mod error;
use error::*;

fn main() -> Result<()> {
    let cmd_line = CmdLine::from_args();
    let config = ConfigFile::load(&cmd_line)?;
    println!("{:?}", config);
    Ok(())
}
