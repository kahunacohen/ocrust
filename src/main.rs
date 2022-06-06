mod args;
mod endpoint;
mod functions;
mod server;

use args::{Commands, OcArgs};
use clap::Parser;

fn main() {
    println!("Hello world!");
    let args = OcArgs::parse();
    match args.command {
        Commands::Papi {method, verbose, path} => {
            println!("method: {:?}", method);
            println!("verbose: {:?}", verbose);
            println!("path: {:?}", path);
        },
        _ => println!("damn")
    }
}
