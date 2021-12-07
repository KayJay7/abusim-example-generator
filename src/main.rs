use args::Opt;
use clap::Parser;

mod args;

fn main() {
    let opt = Opt::parse();
    println!("{:#?}", opt);
}
