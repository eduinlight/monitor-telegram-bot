use crate::utils::make_keyboard;
use std::error::Error;
use teloxide::{
  adaptors::AutoSend,
  prelude::*,
  types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText},
};

pub async fn inline_query_handler(
  q: InlineQuery,
  bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  let choose_debian_version = InlineQueryResultArticle::new(
    "0",
    "Chose debian version",
    InputMessageContent::Text(InputMessageContentText::new("Debian versions:")),
  )
  .reply_markup(make_keyboard());

  bot
    .answer_inline_query(q.id, vec![choose_debian_version.into()])
    .await?;

  Ok(())
}
