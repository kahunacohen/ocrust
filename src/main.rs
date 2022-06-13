mod args;
mod endpoint;
mod functions;
mod papi;
mod server;

use crate::papi::get_server;
use args::{Commands, Method, OcArgs};
use clap::Parser;
use std::process::exit;
use ureq::serde_json::to_string_pretty;
use regex::Regex;


fn main() {
    let args = OcArgs::parse();
    match args.command {
        Commands::Papi {
            method,
            verbose,
            path,
        } => {
            // println!("method: {:?}", method);
            // println!("verbose: {:?}", verbose);
            // println!("path: {:?}", path);
            let server = get_server("https://papi.ourcrowd.com".to_string());
            let re = Regex::new(r"^/|/$").unwrap();
            let normalized_path = format!("/{}/", re.replace_all(&path, ""));
            match server.request(&method.to_string(), normalized_path) {
                Ok(response) => {
                    let json: ureq::serde_json::Value = response.into_json().unwrap();
                    println!("{}", to_string_pretty(&json).unwrap());
                },
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
