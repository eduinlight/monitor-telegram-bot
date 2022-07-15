use dotenv::dotenv;
use std::error::Error;
use teloxide::prelude::*;
#[macro_use]
extern crate lazy_static;

mod callback_handler;
mod globals;
mod inline_query_handler;
mod message_handler;
mod types;
mod utils;

use callback_handler::*;
use inline_query_handler::*;
use message_handler::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  pretty_env_logger::init();
  log::info!("Starting buttons bot...");

  let bot = Bot::from_env().auto_send();

  let handler = dptree::entry()
    .branch(Update::filter_message().endpoint(message_handler))
    .branch(Update::filter_callback_query().endpoint(callback_handler))
    .branch(Update::filter_inline_query().endpoint(inline_query_handler));

  Dispatcher::builder(bot, handler)
    .build()
    .setup_ctrlc_handler()
    .dispatch()
    .await;
  Ok(())
}
