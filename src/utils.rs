use crate::{globals::FILE_PATH, types::AskRequest};
use serde_json::Error;
use std::fs;
use std::io::Write;
use std::{collections::HashMap, fs::File};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn make_keyboard() -> InlineKeyboardMarkup {
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

pub fn save_to_file(ask_requests: &HashMap<i32, AskRequest>) -> () {
  match File::create(FILE_PATH) {
    Ok(mut file) => {
      let json = serde_json::to_string(ask_requests).unwrap_or_default();
      write!(file, "{json}").expect("Error writing to file");
    }
    _ => {}
  }
}

pub fn load_from_file() -> Result<HashMap<i32, AskRequest>, Error> {
  let file_content = fs::read_to_string(FILE_PATH).expect("Error reading from db.json file");
  serde_json::from_str::<HashMap<i32, AskRequest>>(&file_content)
}
