use std::fmt;

use clap::{ArgEnum, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(about, version, author)]
/// Performs debugging and operational related OurCrowd tasks
pub struct OcArgs {
    #[clap(subcommand)]
    pub command: Commands,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Method {
    DELETE,
    GET,
    HEAD,
    OPTION,
    PATCH,
    POST,
    PUT,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Performs Papi operations for debugging and development
    Papi {
        path: String,
        #[clap(short, long, arg_enum, default_value_t = Method::GET)]
        method: Method,
        #[clap(short, long)]
        verbose: bool,
    },
    /// Performs tasks related to rollbar monitoring
    Rollbar { foo: String },
}
