use clap::{Parser, ValueHint};
use std::path::PathBuf;

/// Example generator for abusim
#[derive(Parser, Debug)]
#[clap(name = "aeg", version)]
pub struct Opt {
    /// Output file
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, default_value = "abusim.yml")]
    pub output: PathBuf,

    /// Number of devices
    #[clap(short = 'a', long, default_value = "1")]
    pub devices_number: u32,

    /// Number of rule chains per devices
    #[clap(short = 'b', long, default_value = "1")]
    pub chains_number: u32,

    /// Length of rule chains (internal to a device)
    #[clap(short = 'c', long, default_value = "1")]
    pub chain_length: u32,

    /// Width of (the last level) rule chains
    #[clap(short = 'd', long, default_value = "1")]
    pub chain_width: u32,

    /// Number of devices activated by a chain
    #[clap(short = 'e', long, default_value = "1")]
    pub devices_width: u32,

    /// Length of device chains
    #[clap(short = 'f', long, default_value = "1")]
    pub devices_length: u32,

    /// Agent's docker image
    #[clap(short = 'I', long, default_value = "abulang/abusim-goabu-agent:latest")]
    pub image: String,

    /// Coordinator's docker image
    #[clap(short = 'C', long, default_value = "abulang/abusim-coordinator:latest")]
    pub coordinator_image: String,

    /// Ticking time
    #[clap(short = 'T', long, default_value = "1s")]
    pub tick: String,

    /// Namespace
    #[clap(long, default_value = "abusim-example")]
    pub namespace: String,

    /// Included files
    #[clap(long)]
    pub includes: Vec<String>,

    /// Memory controller
    #[clap(long, default_value = "basic")]
    pub memory_controller: String,

    /// Configuration version
    #[clap(long, default_value = "1.0")]
    pub configuration_version: String,
}

impl Opt {
    pub fn is_valid(&self) -> bool {
        let mut valid = true;
        if self.devices_number == 0 {
            eprintln!("Invalid argument: --devices-number must be at least 1");
            valid = false;
        }

        if self.chains_number == 0 {
            eprintln!("Invalid argument: --chains-number must be at least 1");
            valid = false;
        }

        if self.chain_length == 0 {
            eprintln!("Invalid argument: --chain-length must be at least 1");
            valid = false;
        }

        if self.chain_width == 0 || self.chain_width > self.chains_number {
            eprintln!(
                "Invalid argument: --chain-width must be in between 1 and --chains-number(={})",
                self.chains_number
            );
            valid = false;
        }

        if self.devices_width == 0 {
            eprintln!("Invalid argument: --devices-width must be at least 1");
            valid = false;
        }

        if self.devices_length == 0 {
            eprintln!("Invalid argument: --devices-length must be at least 1");
            valid = false;
        }

        valid
    }
}
