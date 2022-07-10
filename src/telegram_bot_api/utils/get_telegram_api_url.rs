pub fn get_telegram_api_url(token: String) -> String {
  format!("https://api.telegram.org/bot{}", token)
}
