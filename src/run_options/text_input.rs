use std::{ collections::HashMap, io::{ stdin, stdout } };

use colored::Colorize;
use crossterm::{ cursor, queue };

use crate::{
  colorize,
  definitions::{ TerminalColors, TerminalMenuOptions, TextInputEntry },
  util::{ self, clear_rest_of_row, get_current_cursor_row },
};

fn get_text_input(input: String, default_value: &str) -> String {
  let new_line_replaced = input.replace("\n", "");
  return match new_line_replaced.as_str() {
    "" => default_value.to_string(),
    _ => new_line_replaced,
  };
}

fn format_text_input_default(default_value: &str) -> String {
  return format!("({})", default_value.underline());
}

pub fn run_text_input<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: &TerminalColors,
  return_values: &mut HashMap<&'a str, String>,
  entry: &TextInputEntry<'a>
) {
  let text = colorize::paint(
    format!(
      "{} {}: ",
      entry.text,
      format_text_input_default(entry.default)
    ).as_str(),
    &terminal_colors.base_color
  );
  util::print(text, options.indent);
  let mut input = String::new();
  stdin().read_line(&mut input).expect("Did not enter correct string");
  let text_input = get_text_input(input, entry.default);
  let updated_text = format!(
    "{}: {}",
    entry.text,
    colorize::paint(text_input.as_str(), &terminal_colors.selected_option_color)
  );
  queue!(stdout(), cursor::MoveTo(0, get_current_cursor_row() - 1)).unwrap();
  util::print(
    colorize::paint(updated_text.as_str(), &terminal_colors.base_color),
    options.indent
  );
  clear_rest_of_row();
  println!();
  return_values.insert(entry.key, text_input);
}
