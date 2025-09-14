use std::{ collections::HashMap, io::{ stdin, stdout } };

use colored::{ ColoredString, Colorize };
use crossterm::{ cursor, queue };

use crate::{
  colorize,
  definitions::{ BooleanInputEntry, TerminalColors, TerminalMenuOptions },
  util::{ self, clear_rest_of_row, get_current_cursor_row },
};

/// Marks default value as underline uppercase for displaying at end of option row.
fn format_bool_options_text(default_value: bool) -> String {
  return match default_value {
    true => format!("({},n)", "Y".underline()),
    false => format!("(y,{})", "N".underline()),
  };
}

/// Gets translated version based on y/n.
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

/// Returns string representation of given default value.
fn get_bool_default_string(default_value: bool) -> String {
  match default_value {
    true => "y".to_string(),
    false => "n".to_string(),
  }
}

/// Returns string representation of input.
/// Takes into account only first character and ignores casing.
/// In case of incorrect input, falls back to default value.
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

/// Prints initial option text
fn print_option_text<'a>(text: &'a str, default_value: bool, indent: u8) {
  let text = format!("{} {}: ", text, format_bool_options_text(default_value));
  util::print(text.white(), indent);
}

/// Reads user input and converts it to boolean representation.
fn read_bool_input(default_value: bool) -> String {
  let mut input = String::new();
  stdin().read_line(&mut input).expect("Did not enter correct string");
  return get_bool_input(input, default_value);
}

/// Prints the final option text with translated boolean representation.
fn print_final_text<'a>(
  text: &'a str,
  terminal_colors: &TerminalColors,
  indent: u8,
  bool_input: &String
) {
  let updated_text = format!(
    "{}: {}",
    text,
    bool_input_to_text(bool_input, terminal_colors)
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

/// Function that accepts boolean user input and inserts it into given hash map.
///
/// Inserted value will be key value pair of (key, "y"|"n").
pub fn run_bool_input<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: &TerminalColors,
  return_values: &mut HashMap<&'a str, String>,
  entry: &BooleanInputEntry<'a>
) {
  print_option_text(entry.text, entry.default, options.indent);
  let bool_input = read_bool_input(entry.default);
  print_final_text(entry.text, terminal_colors, options.indent, &bool_input);
  return_values.insert(entry.key, bool_input);
}
