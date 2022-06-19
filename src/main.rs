mod args;
mod endpoint;
mod functions;
mod papi;
mod server;

use std::process::exit;
use crate::papi::get_server;
use anyhow;
use args::{Commands, Method, OcArgs};
use clap::Parser;
use endpoint::Payload;
use regex::Regex;
use ureq::serde_json::to_string_pretty;

fn print_json_response(response: ureq::Response) -> anyhow::Result<()> {
    let json: ureq::serde_json::Value = response.into_json()?;
    println!("{}", to_string_pretty(&json)?);
    Ok(())
}
fn main()  {
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
                    if let Err(error) = print_json_response(response) {
                        eprintln!("Error: {}", error);
                        exit(1);
                    } 
                }
                Err(err) => {
                    if err.no_endpoint {
                        eprintln!("Error: no endpoint defined for: '{}'", err.url);
                        exit(1);
                        
                    }
                    eprintln!("{:?}", err);
                    exit(1);
                }
            }
        }
        _ => {
            eprint!("something completely unexpected happened");
            exit(1);
        }
    }
}
