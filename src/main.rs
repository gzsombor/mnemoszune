use structopt::StructOpt;

mod configuration;
use configuration::{CmdLine, ConfigFile};

mod error;
use error::*;

fn main() -> Result<()> {
    let cmd_line = CmdLine::from_args();
    let config = ConfigFile::load(&cmd_line)?;
    if cmd_line.debug {
        let yaml_config = serde_yaml::to_string(&config).map_err(|_e| Error::UnexpectedError)?;
        println!("as yaml:\n {}", &yaml_config);
    }
    println!("{:?}", config);
    Ok(())
}
