mod endpoint;
mod functions;
mod server;
mod args;

use args::OcArgs;
use clap::Parser;

fn main() {
    println!("Hello world!");
    let args = OcArgs::parse();
}
