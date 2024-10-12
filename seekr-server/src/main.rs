use clap::Parser;
use log::info;
use seekr_server::{config::Config, http, logger, BANNER};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // A .env file is optional
    dotenv::dotenv().ok();

    // Initialize the logger.
    // env_logger::init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    // Set the default log level
    logger::init(&config.log);

    info!("{}", BANNER);
    info!("Starting server...");

    let db: sled::Db = sled::open("seekr.db").unwrap();

    // Finally, we spin up our API.
    http::serve(config, db).await?;

    Ok(())
}
