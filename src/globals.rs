use crate::types::AskRequest;
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static! {
  pub static ref HASHMAP: Mutex<HashMap<i32, AskRequest>> = Mutex::new(HashMap::new());
}

pub const FILE_PATH: &str = "./db.json";
