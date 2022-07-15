use crate::{globals::HASHMAP, utils::save_to_file};
use crate::types::*;
use crate::utils::make_keyboard;
use std::error::Error;
use teloxide::{adaptors::AutoSend, prelude::*, types::Message, utils::command::BotCommands, Bot};

pub async fn message_handler(
  m: Message,
  bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  if let Some(text) = m.text() {
    match BotCommands::parse(text, "buttons") {
      Ok(Command::Help) => {
        // Just send the description of all commands.
        bot
          .send_message(m.chat.id, Command::descriptions().to_string())
          .await?;
        bot.delete_message(m.chat.id, m.id).await?;
      }
      Ok(Command::Ask { title }) => {
        // Create a list of buttons and send them.
        let keyboard = make_keyboard();

        let msg_text = if title.is_empty() {
          "Actualización de ventas y rechazos: ".to_string()
        } else {
          (&title).to_string()
        };

        let sended_message = bot
          .send_message(m.chat.id, msg_text.clone())
          .reply_markup(keyboard)
          .await?;

        let mut map = HASHMAP.lock().await;
        map.insert(
          sended_message.id,
          AskRequest {
            message_id: sended_message.id,
            title: msg_text.clone(),
            responses: Vec::new(),
          },
        );

        save_to_file(&map);
      }

      Err(_) => {}
    }
  }

  Ok(())
}
