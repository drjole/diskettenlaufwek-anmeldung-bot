extern crate pretty_env_logger;

use anyhow::Result;
use common::bot;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    log::info!("connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    log::info!("starting bot");
    bot::start(pool).await?;

    Ok(())
}
