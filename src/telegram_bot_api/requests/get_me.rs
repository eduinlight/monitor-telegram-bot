use serde::Deserialize;
use std::error::Error;

use crate::telegram_bot_api::utils::get_telegram_api_url;

#[derive(Deserialize, Debug)]
pub struct BotData {
  pub id: i64,
  pub is_bot: bool,
  pub first_name: String,
  pub username: String,
  pub can_join_groups: bool,
  pub can_read_all_group_messages: bool,
  pub supports_inline_queries: bool,
}

#[derive(Deserialize, Debug)]
pub struct GetMeApiResponse {
  pub ok: bool,
  pub result: BotData,
}

pub async fn get_me(token: String) -> Result<GetMeApiResponse, Box<dyn Error>> {
  let response = reqwest::get(format!("{}/getMe", get_telegram_api_url(token.clone())))
    .await?
    .json::<GetMeApiResponse>()
    .await?;

  Ok(response)
}
