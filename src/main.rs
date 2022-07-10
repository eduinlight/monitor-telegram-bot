use dotenv::dotenv;
use std::error::Error;

mod telegram_bot_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  pretty_env_logger::init();
  log::info!("Starting buttons bot...");

  Ok(())
}
