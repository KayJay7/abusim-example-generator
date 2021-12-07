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
    let file = File::create(opt.output);
    match file {
        Ok(file) => {
            serde_yaml::to_writer(file, &Config::new()).unwrap();
            #[cfg(debug_assertions)]
            println!("{}", serde_yaml::to_string(&Config::new()).unwrap());
        }
        Err(code) => {
            eprintln!("Unable to create file: {}", code);
        }
    }
}
