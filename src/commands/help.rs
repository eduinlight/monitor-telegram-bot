use std::error::Error;

use teloxide::{dispatching::DpHandlerDescription, prelude::*, utils::command::BotCommands};

use super::Command;

pub async fn help_endpoint(
  m: Message,
  bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  bot
    .send_message(m.chat.id, Command::descriptions().to_string())
    .await?;
  Ok(())
}

pub async fn help_handler(
) -> Handler<'static, DependencyMap, Result<(), Box<dyn Error + Send + Sync>>, DpHandlerDescription>
{
  Update::filter_message()
    .filter_command::<Command>()
    .branch(dptree::case![Command::Help].endpoint(help_endpoint))
}
