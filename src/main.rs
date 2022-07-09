use dotenv::dotenv;
use std::error::Error;
use teloxide::{
  payloads::SendMessageSetters,
  prelude::*,
  types::{
    InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
    InputMessageContentText,
  },
  utils::command::BotCommands,
};

#[derive(BotCommands)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
  #[command(description = "Display this text")]
  Help,
  #[command(description = "Start")]
  Start,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  pretty_env_logger::init();
  log::info!("Starting buttons bot...");

  let bot = Bot::from_env().auto_send();

  let handler = dptree::entry()
    .branch(Update::filter_message().endpoint(message_handler))
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

/// Creates a keyboard made by buttons in a big column.
fn make_keyboard() -> InlineKeyboardMarkup {
  let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

  let debian_versions = [
    "Responder"
  ];

  for versions in debian_versions.chunks(3) {
    let row = versions
      .iter()
      .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
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
      }
      Ok(Command::Start) => {
        // Create a list of buttons and send them.
        let keyboard = make_keyboard();
        bot
          .send_message(m.chat.id, "Debian versions:")
          .reply_markup(keyboard)
          .await?;
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
  if let Some(version) = q.data {
    let text = format!("You chose: {version}");

    match q.message {
      Some(Message { id, chat, .. }) => {
        bot.edit_message_text(chat.id, id, text).await?;
      }
      None => {
        if let Some(id) = q.inline_message_id {
          bot.edit_message_text_inline(id, text).await?;
        }
      }
    }

    log::info!("You chose: {}", version);
  }

  Ok(())
}
