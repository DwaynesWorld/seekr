use crate::{logger::Level, BANNER};

pub const LOG: &str = "seekr::server";
pub const INFO: &str = "A powerful and flexible user interface for managing Apache Kafka. Handle your day-to-day tasks with ease, find exactly what you're looking for, and fix issues quickly.";

/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
/// The latter is preferred as environment variables are one of the recommended ways to
/// get configuration from Kubernetes Secrets in deployment.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
#[derive(clap::Parser)]
#[clap(name = "Seekr server command-line interface")]
#[clap(about = INFO, before_help = BANNER, disable_version_flag = true, arg_required_else_help = true)]
pub struct Config {
    #[clap(
        long,
        env = "SEEKR_LOG",
        default_value = "info",
        help = "The logging level",
        value_enum
    )]
    /// The logging level
    pub log: Level,

    #[clap(
        long,
        env = "SEEKR_HOST",
        default_value = "localhost",
        help = "the host address to bind on"
    )]
    /// The host address to bind on
    pub host: String,

    #[clap(
        long,
        env = "SEEKR_PORT",
        default_value = "5000",
        help = "the port the HTTP server will listen on"
    )]
    /// The port the HTTP server will listen on
    pub port: i32,
}
