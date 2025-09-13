use std::{ collections::HashMap, io::{ stdin, stdout } };
use colored::Colorize;
use crossterm::{ cursor, queue };

use crate::{
  colorize,
  definitions::{ InputEntry, TerminalMenuOptions },
  util::{ self, clear_rest_of_row, get_current_cursor_row },
};

fn bool_input_to_text(bool_input: &String) -> String {
  return match bool_input.as_str() {
    "y" => "Yes".to_string(),
    "n" => "No".to_string(),
    _ => {
      panic!("Unknown boolean input: {}", bool_input);
    }
  };
}

fn get_bool_input(input: String, default_value: bool) -> String {
  return match input.as_str() {
    "y\n" => "y".to_string(),
    "n\n" => "n".to_string(),
    _ => {
      match default_value {
        true => "y".to_string(),
        false => "n".to_string(),
      }
    }
  };
}

pub fn run<'a>(options: &TerminalMenuOptions<'a>) -> HashMap<&'a str, String> {
  let mut return_values: HashMap<&'a str, String> = HashMap::new();
  options.input_entries.iter().for_each(|entry| {
    match entry {
      InputEntry::BOOL(entry) => {
        let text = format!("{} (y/n): ", entry.text);
        util::print(text.white(), options.indent);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Did not enter correct string");
        let bool_input = get_bool_input(input, entry.default);
        let updated_text = format!(
          "{}: {}",
          entry.text,
          bool_input_to_text(&bool_input)
        );
        queue!(
          stdout(),
          cursor::MoveTo(0, get_current_cursor_row() - 1)
        ).unwrap();
        util::print(
          colorize::paint(updated_text.as_str(), &options.base_color),
          options.indent
        );
        clear_rest_of_row();
        println!();
        return_values.insert(entry.key, bool_input);
      }
      InputEntry::CHOOSABLE(entry) => {
        let text = format!("{} (choose an option): ", entry.text);
        util::print(
          colorize::paint(text.as_str(), &options.base_color),
          options.indent
        );
        print!("{:?}", cursor::position());
      }
    }
  });
  return return_values;
}
