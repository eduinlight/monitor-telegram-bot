use teloxide::utils::command::BotCommands;

#[derive(BotCommands)]
#[command(
  rename = "lowercase",
  description = "Este bot soporta los siguientes comandos:"
)]
pub enum Command {
  #[command(description = "Muestra este mensaje de ayuda")]
  Help,
  #[command(description = "Generar pedido de ventas")]
  AskForSells,
}
