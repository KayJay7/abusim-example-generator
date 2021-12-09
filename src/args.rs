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
}
