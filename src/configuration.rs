use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

use crate::error::*;

/// Smart web proxy
#[derive(StructOpt, Debug)]
#[structopt(name = "mnémoszüné")]
pub struct CmdLine {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Configuration file
    #[structopt(short, long, parse(from_os_str))]
    config: PathBuf,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    port: u16,
    unsecure_port: u16,

    #[serde(default)]
    defaults: ServiceDefaults,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefaults {
    keep_alive_min: u16,
}

impl Default for ServiceDefaults {
    fn default() -> Self {
        ServiceDefaults { keep_alive_min: 15 }
    }
}

impl ConfigFile {
    fn load_path(path: &PathBuf) -> Result<ConfigFile> {
        let file = std::fs::File::open(path).map_err(|e| Error::LoadConfiguration {
            source: e,
            path: path.clone(),
        })?;
        let config_file: ConfigFile =
            serde_yaml::from_reader(file).map_err(|e| Error::InvalidConfiguration {
                source: e,
                path: path.clone(),
            })?;
        Ok(config_file)
    }
    pub fn load(cmd_line: &CmdLine) -> Result<ConfigFile> {
        ConfigFile::load_path(&cmd_line.config)
    }
}
