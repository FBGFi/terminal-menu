use std::collections::HashMap;

use crate::{ definitions::{ TerminalColors, TerminalMenuOptions } };

mod print_base_texts;
mod util;
mod colorize;
mod run_options;
pub mod definitions;

/// Runs a wizard-like menu that can be used for program setups.
///
/// Note: duplicate keys for input entries are not enforced,
/// it is callers responsibility to make sure keys are unique.
///
/// Takes in:
/// * base options
/// * terminal colorings
///
/// Returns:
/// * hash map of key-value pairs from given input entries
pub fn run_terminal_menu<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: TerminalColors
) -> HashMap<&'a str, String> {
  println!();
  print_base_texts::print_header(
    options.header_text,
    &terminal_colors.base_color,
    options.indent
  );
  println!();
  print_base_texts::print_description(&options.description, options.indent);
  println!();
  let return_values = run_options::run(&options, &terminal_colors);
  println!();
  return return_values;
}

/// Runs a wizard-like menu that can be used for program setups.
///
/// Note: duplicate keys for input entries are not enforced,
/// it is callers responsibility to make sure keys are unique.
///
/// Takes in:
/// * base options
/// * optional colorings, if none passed defaults will be used
///
/// Returns:
/// * hash map of key-value pairs from given input entries
#[macro_export]
macro_rules! run_terminal_menu {
  ($options:expr) => {
    run_terminal_menu(&$options, $crate::definitions::TerminalColors {
        base_color: $crate::definitions::ColorOptions::BLUE,
        selected_option_color: $crate::definitions::ColorOptions::GREEN,
        falsy_selection_color: $crate::definitions::ColorOptions::RED,
      })
  };
  ($options:expr, $terminal_colors:expr) => {
    run_terminal_menu(&$options, $terminal_colors)
  };
  ($options:expr) => {
    run_terminal_menu($options, $crate::definitions::TerminalColors {
        base_color: $crate::definitions::ColorOptions::BLUE,
        selected_option_color: $crate::definitions::ColorOptions::GREEN,
        falsy_selection_color: $crate::definitions::ColorOptions::RED,
      })
  };
  ($options:expr, $terminal_colors:expr) => {
    run_terminal_menu($options, $terminal_colors)
  };
}
