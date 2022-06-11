mod args;
mod endpoint;
mod functions;
mod papi;
mod server;

use crate::papi::get_server;
use args::{Commands, Method, OcArgs};
use clap::Parser;
use std::process::exit;

fn main() {
    let args = OcArgs::parse();
    match args.command {
        Commands::Papi {
            method,
            verbose,
            path,
        } => {
            println!("method: {:?}", method);
            println!("verbose: {:?}", verbose);
            println!("path: {:?}", path);
            let server = get_server("https://papi.ourcrowd.com".to_string());

            match server.request("get", path) {
                Ok(_) => println!("Succeeded!"),
                Err(err) => {
                    if err.no_endpoint {
                        eprintln!("Error: no endpoint defined for: '{}'", err.url);
                        std::process::exit(1);
                    }
                    println!("{:?}", err)
                }
            }
        }
        _ => println!("damn"),
    }
}
