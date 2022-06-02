use clap::{ArgEnum, Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
/// Performs debugging and operational related OurCrowd tasks
pub struct OcArgs {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Method {
    DELETE,
    GET,
    HEAD,
    OPTION,
    PATCH,
    POST,
    PUT,
}
#[derive(Subcommand)]
enum Commands {
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
