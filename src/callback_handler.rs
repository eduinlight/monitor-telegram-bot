use crate::types::*;
use crate::utils::*;
use crate::{globals::HASHMAP, types::AskRequest};
use std::error::Error;
use teloxide::{
  adaptors::AutoSend,
  prelude::*,
  types::{CallbackQuery, Message},
  Bot,
};

pub async fn callback_handler(
  q: CallbackQuery,
  bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  if let Some(option) = q.data {
    match q.message.clone() {
      Some(Message { id, chat, .. }) => {
        let mut map = HASHMAP.lock().await;
        if map.contains_key(&id) {
          let n = option.parse::<i32>().unwrap();

          let mut question: AskRequest = map
            .get_mut(&id)
            .map(|v| v.clone())
            .unwrap();

          let user = q.from;
          let mut found = false;
          log::info!("ACA {n}");
          question.responses = question
            .responses
            .iter()
            .map(|r| {
              if user.clone().id == r.user.id {
                found = true;
                Ask {
                  sells: if n % 2 == 0 {
                    Some(n / 2)
                  } else {
                    r.clone().sells
                  },
                  refuse: if n % 2 == 1 {
                    Some(n / 2)
                  } else {
                    r.clone().refuse
                  },
                  user: user.clone(),
                }
              } else {
                r.clone()
              }
            })
            .collect::<Vec<Ask>>();

          if !found {
            question.responses.push(Ask {
              sells: if n % 2 == 0 { Some((n + 1) / 2) } else { None },
              refuse: if n % 2 == 1 { Some((n + 1) / 2) } else { None },
              user: user.clone(),
            })
          }

          map.insert(id, question.clone());
          save_to_file(&map);

          let keyboard = make_keyboard();
          bot
            .edit_message_text(chat.id, question.message_id, format!("{question}"))
            .reply_markup(keyboard)
            .await?;
        }
      }
      _ => {}
    }
  }

  Ok(())
}
