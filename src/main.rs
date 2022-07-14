use dotenv::dotenv;
use std::{collections::HashMap, error::Error, fmt::Display};
use teloxide::{
  payloads::SendMessageSetters,
  prelude::*,
  types::{
    InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
    InputMessageContentText, User,
  },
  utils::command::BotCommands,
};
use tokio::sync::Mutex;
#[macro_use]
extern crate lazy_static;

#[derive(Clone)]
struct Ask {
  sells: Option<i32>,
  refuse: Option<i32>,
  user: User,
}

#[derive(Clone)]
struct AskRequest {
  message_id: i32,
  title: String,
  responses: Vec<Ask>,
}

impl Display for Ask {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{} Ventas: {} Rechasos: {}",
      self.user.to_owned().first_name,
      if self.sells.is_some() {
        self.sells.unwrap().to_string()
      } else {
        0.to_string()
      },
      if self.refuse.is_some() {
        self.refuse.unwrap().to_string()
      } else {
        0.to_string()
      }
    )
  }
}

impl Display for AskRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let resp: String = self.title.to_owned();
    write!(
      f,
      "{}",
      format!(
        "{}\n\n{}",
        resp,
        self
          .responses
          .iter()
          .map(|r| format!("{r}"))
          .collect::<Vec<String>>()
          .join("\n")
      )
    )
  }
}

lazy_static! {
  static ref HASHMAP: Mutex<HashMap<i32, AskRequest>> = Mutex::new(HashMap::new());
}

#[derive(BotCommands)]
#[command(
  rename = "lowercase",
  description = "Este bot es para ayudar en el control de ventas y rechasos."
)]
enum Command {
  #[command(description = "Muestra esta ayuda")]
  Help,
  #[command(description = "Preguntar por ventas")]
  Ask { title: String },
}

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

fn make_keyboard() -> InlineKeyboardMarkup {
  let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

  for arr in (0..42).collect::<Vec<i32>>().chunks(2) {
    let row = arr
      .iter()
      .map(|i| {
        InlineKeyboardButton::callback(
          if i % 2 == 0 {
            format!("Ventas {}", i / 2)
          } else {
            format!("Rechasos {}", i / 2)
          },
          i.to_string(),
        )
      })
      .collect();

    keyboard.push(row);
  }

  InlineKeyboardMarkup::new(keyboard)
}

async fn message_handler(
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
        HASHMAP.lock().await.insert(
          sended_message.id,
          AskRequest {
            message_id: sended_message.id,
            title: msg_text.clone(),
            responses: Vec::new(),
          },
        );
      }

      Err(_) => {
        bot.send_message(m.chat.id, "Command not found!").await?;
      }
    }
  }

  Ok(())
}

async fn inline_query_handler(
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

async fn callback_handler(
  q: CallbackQuery,
  bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  if let Some(option) = q.data {
    match q.message.clone() {
      Some(Message { id, chat, .. }) => {
        if HASHMAP.lock().await.contains_key(&id) {
          let n = option.parse::<i32>().unwrap();

          let mut question: AskRequest = HASHMAP
            .lock()
            .await
            .get_mut(&id)
            .map(|v| v.clone())
            .unwrap();

          let user = q.from;
          let mut found = false;
          question.responses = question
            .responses
            .iter()
            .map(|r| {
              if user.clone().id == r.user.id {
                found = true;
                Ask {
                  sells: if n % 2 == 0 {
                    Some((n + 1) / 2)
                  } else {
                    r.clone().sells
                  },
                  refuse: if n % 2 == 1 {
                    Some((n + 1) / 2)
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

          HASHMAP.lock().await.insert(id, question.clone());

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
