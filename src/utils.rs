use crate::{globals::FILE_PATH, types::AskRequest};
use serde_json::Error;
use std::fs;
use std::io::Write;
use std::{collections::HashMap, fs::File};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use xlsxwriter::{FormatAlignment, FormatColor, FormatUnderline, Workbook};

pub fn make_keyboard() -> InlineKeyboardMarkup {
  let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

  for arr in (0..32).collect::<Vec<i32>>().chunks(4) {
    let row = arr
      .iter()
      .map(|i| {
        InlineKeyboardButton::callback(
          if *i < 16 {
            format!("👌 V {}", i)
          } else {
            format!("🚫 R {}", i - 16)
          },
          i.to_string(),
        )
      })
      .collect();

    keyboard.push(row);
  }

  InlineKeyboardMarkup::new(keyboard)
}

pub fn save_to_file(ask_requests: &HashMap<i32, AskRequest>) {
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

pub fn create_excel(question: &AskRequest) -> InputFile {
  let file_name = format!("../Reporte {}.xlsx", question.message_id);
  let workbook = Workbook::new(&file_name);

  let title_format = workbook
    .add_format()
    .set_font_size(12.0)
    .set_bold()
    .set_align(FormatAlignment::Left)
    .set_align(FormatAlignment::VerticalCenter);

  let header_format = workbook
    .add_format()
    .set_bold()
    .set_bg_color(FormatColor::Gray);

  let align_center = workbook.add_format().set_align(FormatAlignment::Center);
  let align_left = workbook.add_format().set_align(FormatAlignment::Left);

  let mut sheet1 = workbook.add_worksheet(None).unwrap();
  let mut row = 0;
  // format
  sheet1.set_row(0, 35.0, None).unwrap();
  sheet1.set_column(0, 3, 7.0, Some(&align_center)).unwrap();
  sheet1.set_column(0, 0, 7.0, Some(&align_left)).unwrap();
  sheet1.set_column(1, 1, 50.0, Some(&align_left)).unwrap();
  // title
  sheet1
    .merge_range(
      row,
      0,
      row,
      3,
      &format!("Reporte número: {}", question.message_id),
      Some(&title_format),
    )
    .unwrap();

  row += 1;

  // header
  sheet1
    .write_string(row, 0, "No.", Some(&header_format))
    .unwrap();
  sheet1
    .write_string(row, 1, "Nomber y apellidos", Some(&header_format))
    .unwrap();
  sheet1
    .write_string(row, 2, "Ventas", Some(&header_format))
    .unwrap();
  sheet1
    .write_string(row, 3, "Rechazos", Some(&header_format))
    .unwrap();
  row += 1;

  // data
  for response in question.responses.iter() {
    sheet1
      .write_number(row, 0, f64::from(row) - 1.0, None)
      .unwrap();
    sheet1
      .write_string(row, 1, &response.get_full_name(), None)
      .unwrap();
    sheet1
      .write_string(row, 2, &response.sells_to_string(), None)
      .unwrap();
    sheet1
      .write_string(row, 3, &response.refuse_to_string(), None)
      .unwrap();
    row += 1;
  }

  sheet1.set_tab_color(FormatColor::Cyan);
  workbook.close().unwrap();
  InputFile::file(file_name)
}
