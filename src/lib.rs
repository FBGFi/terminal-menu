use std::collections::HashMap;

use crate::{
  definitions::{ TerminalColors, TerminalMenuOptions },
  util::get_terminal_colors,
};

mod print_base_texts;
mod stylize;
mod util;
mod colorize;
mod run_options;
pub mod definitions;

pub fn run_terminal_menu<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: Option<TerminalColors>
) -> HashMap<&'a str, String> {
  println!();
  let used_colors = get_terminal_colors(terminal_colors);
  print_base_texts::print_header(
    options.header_text,
    &used_colors.base_color,
    options.indent
  );
  println!();
  print_base_texts::print_description(&options.description, options.indent);
  println!();
  let return_values = run_options::run(&options, &used_colors);
  println!();
  return return_values;
}
