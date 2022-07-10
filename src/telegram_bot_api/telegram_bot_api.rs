use std::error::Error;

use super::requests::get_me::{BotData, get_me};

#[derive(Debug)]
pub struct TelegramBotApi {
  pub token: String,
  pub bot_data: BotData,
}

impl TelegramBotApi {
  pub async fn new(token: String) -> Result<TelegramBotApi, Box<dyn Error>> {
    let response = get_me(token.clone()).await?;
    Ok(TelegramBotApi { token, bot_data:  response.result})
  }
}
