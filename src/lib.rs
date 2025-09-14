use std::collections::HashMap;

use crate::{ definitions::{ TerminalColors, TerminalMenuOptions } };

mod print_base_texts;
mod stylize;
mod util;
mod colorize;
mod run_options;
pub mod definitions;

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

#[macro_export]
macro_rules! run_terminal_menu {
  ($options:expr) => {
    run_terminal_menu($options, $crate::definitions::TerminalColors {
        base_color: $crate::definitions::ColorOptions::BLUE,
        selected_option_color: $crate::definitions::ColorOptions::GREEN,
        falsy_selection_color: $crate::definitions::ColorOptions::RED,
      })
  };
  ($options:expr, $terminal_colors:expr) => {
    run_terminal_menu($options,$terminal_colors)
  };
}
