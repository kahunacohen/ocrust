mod args;
mod endpoint;
mod functions;
mod papi;
mod server;

use crate::papi::get_server;
use anyhow;
use args::{Commands, Method, OcArgs};
use clap::{Error, ErrorKind, Parser};
use endpoint::Payload;
use regex::Regex;
use std::{any, process::exit};
use ureq::serde_json::to_string_pretty;

fn run_papi_req(method: String, path: String) -> anyhow::Result<()> {
    let server = get_server("https://papi.ourcrowd.com".to_string());
    let re = Regex::new(r"^/|/$").unwrap();
    let normalized_path = format!("/{}/", re.replace_all(&path, ""));
    let response = server.request(&method.to_string(), normalized_path)?;
    let json: ureq::serde_json::Value = response.into_json()?;
    println!("{}", to_string_pretty(&json)?);
    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    let args = OcArgs::parse();
    match args.command {
        Commands::Papi {
            method,
            verbose,
            path,
        } => {
            run_papi_req(method.to_string(), path)?;
            Ok(())
        }
        _ => {
            // This should be handled by clap..but need wildcard to be exaustive.
            Err(anyhow::anyhow!("unexpected subcommand"))
        }
    }
}
