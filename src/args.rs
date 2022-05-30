use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
pub struct OcArgs {
   #[clap(subcommand)]
   command: Commands,
}

#[derive(Subcommand)]
enum Commands {
   /// Performs operations related to papi
   Papi { number_one: i32, number_two: i32 }
}