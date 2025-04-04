use color_eyre::{Result, eyre::Context};

mod cli;
mod db;
mod logging;
mod stats;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::cli();

    logging::setup_logging(&cli).wrap_err_with(|| "Failed to setup logging")?;

    let db = db::Db::new(&cli.sqlite_db)
        .await
        .wrap_err_with(|| "Failed to connect to database")?;

    web::run(cli, db)
        .await
        .wrap_err_with(|| "Failed to run web server")?;

    Ok(())
}
