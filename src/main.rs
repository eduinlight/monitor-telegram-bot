use dotenv::dotenv;
use std::error::Error;
use teloxide::prelude::*;

mod commands;

use commands::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  pretty_env_logger::init();
  log::info!("Starting buttons bot...");

  let bot = Bot::from_env().auto_send();

  let handler = dptree::entry().branch(help_handler().await);

  Dispatcher::builder(bot, handler)
    .build()
    .setup_ctrlc_handler()
    .dispatch()
    .await;
  Ok(())
}
