use std::{ collections::HashMap, io::{ stdin, stdout } };

use colored::Colorize;
use crossterm::{ cursor, queue };

use crate::{
  colorize,
  definitions::{ TerminalColors, TerminalMenuOptions, TextInputEntry },
  util::{ self, clear_rest_of_row, get_current_cursor_row },
};

/// Removes new line character after input and defaults to given default value in case of no input.
fn get_text_input(input: String, default_value: &str) -> String {
  let new_line_replaced = input.replace("\n", "");
  return match new_line_replaced.as_str() {
    "" => default_value.to_string(),
    _ => new_line_replaced,
  };
}

/// Formats default value to be shown in initial option text.
fn format_text_input_default(default_value: &str) -> String {
  return format!("({})", default_value.underline());
}

/// Prints initial option text.
fn print_option_text<'a>(text: &'a str, default_value: &'a str, indent: u8) {
  let text = format!("{} {}: ", text, format_text_input_default(default_value));
  util::print(text.white(), indent);
}

/// Reads user input, removes new line character and falls back to default if empty input.
fn read_text_input<'a>(default_value: &'a str) -> String {
  let mut input = String::new();
  stdin().read_line(&mut input).expect("Did not enter correct string");
  return get_text_input(input, default_value);
}

/// Prints the final option text with given user input.
fn print_final_text<'a>(
  text: &'a str,
  terminal_colors: &TerminalColors,
  indent: u8,
  text_input: &String
) {
  let updated_text = format!(
    "{}: {}",
    text,
    colorize::paint(text_input.as_str(), &terminal_colors.selected_option_color)
  );

  // Replace original option text
  queue!(stdout(), cursor::MoveTo(0, get_current_cursor_row() - 1)).unwrap();
  util::print(
    colorize::paint(updated_text.as_str(), &terminal_colors.base_color),
    indent
  );
  clear_rest_of_row();

  // Move cursor to new row
  println!();
}

/// Function that accepts user input and inserts it into given hash map.
///
/// Inserted value will be key value pair of (key, String).
pub fn run_text_input<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: &TerminalColors,
  return_values: &mut HashMap<&'a str, String>,
  entry: &TextInputEntry<'a>
) {
  print_option_text(entry.text, entry.default, options.indent);
  let text_input = read_text_input(entry.default);
  print_final_text(entry.text, terminal_colors, options.indent, &text_input);
  return_values.insert(entry.key, text_input);
}
