use std::error::Error;

use super::requests::get_me::{BotData, get_me};

pub struct TelegramBotApi {
  token: String,
  bot_data: BotData,
}

impl TelegramBotApi {
  async fn new(token: String) -> Result<TelegramBotApi, Box<dyn Error>> {
    let response = get_me(token).await?;
    Ok(TelegramBotApi { token, bot_data: })
  }
}
