use tokio::sync::Mutex;

use crate::{types::AskRequest, utils::load_from_file};
use std::collections::HashMap;

lazy_static! {
  pub static ref HASHMAP: Mutex<HashMap<i32, AskRequest>> = {
    match load_from_file() {
      Ok(map) => Mutex::new(map),
      _ => Mutex::new(HashMap::new()),
    }
  };
}

pub const FILE_PATH: &str = "../db.json";
