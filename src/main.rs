mod args;
mod endpoint;
mod functions;
mod server;

use args::OcArgs;
use clap::Parser;

fn main() {
    println!("Hello world!");
    let args = OcArgs::parse();
}
