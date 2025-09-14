use std::{ collections::HashMap, io::{ stdin, stdout } };

use colored::{ ColoredString, Colorize };
use crossterm::{ cursor, queue };

use crate::{
  colorize,
  definitions::{ BooleanInputEntry, TerminalColors, TerminalMenuOptions },
  util::{ self, clear_rest_of_row, get_current_cursor_row },
};

fn format_bool_options_text(default_value: bool) -> String {
  return match default_value {
    true => format!("({},n)", "Y".underline()),
    false => format!("(y,{})", "N".underline()),
  };
}

fn bool_input_to_text(
  bool_input: &String,
  terminal_colors: &TerminalColors
) -> ColoredString {
  return match bool_input.as_str() {
    "y" => colorize::paint("Yes", &terminal_colors.selected_option_color),
    "n" => colorize::paint("No", &terminal_colors.falsy_selection_color),
    _ => {
      panic!("Unknown boolean input: {}", bool_input);
    }
  };
}

fn get_bool_default_string(default_value: bool) -> String {
  match default_value {
    true => "y".to_string(),
    false => "n".to_string(),
  }
}

fn get_bool_input(input: String, default_value: bool) -> String {
  let first_char = input.chars().nth(0);
  return match first_char {
    Some(char) => {
      match char.to_ascii_lowercase() {
        'y' => "y".to_string(),
        'n' => "n".to_string(),
        _ => get_bool_default_string(default_value),
      }
    }
    None => get_bool_default_string(default_value),
  };
}

pub fn run_bool_input<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: &TerminalColors,
  return_values: &mut HashMap<&'a str, String>,
  entry: &BooleanInputEntry<'a>
) {
  let text = format!(
    "{} {}: ",
    entry.text,
    format_bool_options_text(entry.default)
  );
  util::print(text.white(), options.indent);
  let mut input = String::new();
  stdin().read_line(&mut input).expect("Did not enter correct string");
  let bool_input = get_bool_input(input, entry.default);
  let updated_text = format!(
    "{}: {}",
    entry.text,
    bool_input_to_text(&bool_input, terminal_colors)
  );
  queue!(stdout(), cursor::MoveTo(0, get_current_cursor_row() - 1)).unwrap();
  util::print(
    colorize::paint(updated_text.as_str(), &terminal_colors.base_color),
    options.indent
  );
  clear_rest_of_row();
  println!();
  return_values.insert(entry.key, bool_input);
}
