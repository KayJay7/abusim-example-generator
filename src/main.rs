use args::Opt;
use clap::Parser;
use generator::Config;
use std::fs::File;

mod args;
mod generator;

fn main() {
    let opt = Opt::parse();
    #[cfg(debug_assertions)]
    println!("{:#?}", opt);
    let file = File::create(&opt.output);
    match file {
        Ok(file) => {
            let config = Config::from(opt);
            serde_yaml::to_writer(file, &config).unwrap();
            #[cfg(debug_assertions)]
            println!("{}", serde_yaml::to_string(&config).unwrap());
        }
        Err(code) => {
            eprintln!("Unable to create file: {}", code);
        }
    }
}
