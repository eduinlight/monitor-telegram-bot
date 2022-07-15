use serde::{Deserialize, Serialize};
use std::fmt::Display;
use teloxide::{types::User, utils::command::BotCommands};

#[derive(Clone, Serialize, Deserialize)]
pub struct Ask {
  pub sells: Option<i32>,
  pub refuse: Option<i32>,
  pub user: User,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AskRequest {
  pub message_id: i32,
  pub title: String,
  pub responses: Vec<Ask>,
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

#[derive(BotCommands)]
#[command(
  rename = "lowercase",
  description = "Este bot es para ayudar en el control de ventas y rechasos."
)]
pub enum Command {
  #[command(description = "Muestra esta ayuda")]
  Help,
  #[command(description = "Preguntar por ventas")]
  Ask { title: String },
}
