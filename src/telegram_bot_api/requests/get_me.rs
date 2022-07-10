use serde::Deserialize;
use std::error::Error;

use crate::telegram_bot_api::utils::get_telegram_api_url;

#[derive(Deserialize)]
pub struct BotData {
  id: i64,
  is_bot: bool,
  first_name: String,
  username: String,
  can_join_groups: bool,
  can_read_all_group_messages: bool,
  supports_inline_queries: bool,
}

#[derive(Deserialize)]
pub struct GetMeApiResponse {
  ok: bool,
  result: BotData,
}

pub async fn get_me(token: String) -> Result<GetMeApiResponse, Box<dyn Error>> {
  let response = reqwest::get(get_telegram_api_url(token))
    .await?
    .json::<GetMeApiResponse>()
    .await?;

  Ok(response)
}
