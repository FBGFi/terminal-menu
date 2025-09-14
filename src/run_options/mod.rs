use std::{ collections::HashMap };

use crate::{
  definitions::{ InputEntry, TerminalColors, TerminalMenuOptions },
  run_options::{
    bool_input::run_bool_input,
    choosable_input::run_choosable_input,
    text_input::run_text_input,
  },
};

mod bool_input;
mod text_input;
mod choosable_input;

/// Chooses input mode based on currently active option.
fn run_input_mode<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: &TerminalColors,
  return_values: &mut HashMap<&'a str, String>,
  entry: &InputEntry<'a>
) {
  match entry {
    InputEntry::BOOL(entry) => {
      run_bool_input(options, terminal_colors, return_values, entry);
    }
    InputEntry::TEXT(entry) => {
      run_text_input(options, terminal_colors, return_values, entry);
    }
    InputEntry::CHOOSABLE(entry) => {
      run_choosable_input(options, terminal_colors, return_values, entry);
    }
  }
}

/// Iterates through given options and enters corresponding input mode for each entry.
pub fn run<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: &TerminalColors
) -> HashMap<&'a str, String> {
  let mut return_values: HashMap<&'a str, String> = HashMap::new();
  options.input_entries.iter().for_each(|entry| {
    run_input_mode(options, terminal_colors, &mut return_values, entry);
  });
  return return_values;
}
