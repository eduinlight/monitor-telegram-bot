use dotenv::dotenv;
use log::info;
use std::env;
use std::error::Error;

mod telegram_bot_api;

use telegram_bot_api::TelegramBotApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  pretty_env_logger::init();
  info!("Starting...");

  let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN need to be in environment");

  let bot: TelegramBotApi = TelegramBotApi::new(token).await?;

  info!("{}", format!("{:?}", bot));

  Ok(())
}
