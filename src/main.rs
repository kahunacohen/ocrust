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

fn process_json(server: server::Server<Payload>, method: Method, normalized_path: String) -> anyhow::Result<()> {
    let response = server.request(&method.to_string(), normalized_path)?;
    let json: ureq::serde_json::Value = response.into_json()?;
    println!("{}", to_string_pretty(&json)?);
    Ok(())
}
fn main() -> () {
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
                    if let Err(error) = process_json(server, method, normalized_path) {
                        eprintln!("Error: {}", error);
                        
                    } else {
                        
                    }
                }
                Err(err) => {
                    if err.no_endpoint {
                        eprintln!("Error: no endpoint defined for: '{}'", err.url);
                        
                    }
                    eprintln!("{:?}", err);
                    
                }
            }
        }
        _ => {
            println!("something completely unexpected happened");
            
        }
    }
}
