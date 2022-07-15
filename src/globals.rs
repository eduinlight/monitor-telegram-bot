use tokio::sync::Mutex;

use crate::types::AskRequest;
use std::collections::HashMap;

lazy_static! {
  pub static ref HASHMAP: Mutex<HashMap<i32, AskRequest>> = Mutex::new(HashMap::new());
  pub static ref FILE_PATH: Mutex<String> = Mutex::new(String::from(""));
}
