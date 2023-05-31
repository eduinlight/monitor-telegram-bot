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

impl Ask {
  pub fn get_full_name(&self) -> String {
    format!(
      "{} {}",
      String::from(self.user.to_owned().first_name),
      String::from(self.user.to_owned().last_name.unwrap_or_default())
    )
  }

  pub fn sells_to_string(&self) -> String {
    if self.sells.is_some() {
      self.sells.unwrap().to_string()
    } else {
      0.to_string()
    }
  }

  pub fn refuse_to_string(&self) -> String {
    if self.refuse.is_some() {
      self.refuse.unwrap().to_string()
    } else {
      0.to_string()
    }
  }
}

impl Display for Ask {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{} Ventas: {} Rechazos: {}",
      self.user.to_owned().first_name,
      self.sells_to_string(),
      self.refuse_to_string()
    )
  }
}

impl Display for AskRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let resp: String = self.title.to_owned();
    let mut responses = self.responses.to_owned();

    responses.sort_by(|a, b| a.user.first_name.cmp(&b.user.first_name));

    write!(
      f,
      "{}",
      format!(
        "📝 {} \n{}\n\n{}",
        self.message_id,
        resp,
        responses
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
  // #[command(description = "Muestra esta ayuda")]
  // Help,
  #[command(description = "Preguntar por ventas")]
  Ask { title: String },
  #[command(description = "Generar reporte")]
  Report { chat_id: i32 },
}
