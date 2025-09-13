use std::collections::HashMap;

use crate::definitions::{ TerminalMenuOptions };

mod print_menu_header;
mod stylize;
mod util;
mod colorize;
mod run_options;
pub mod definitions;

pub fn run_terminal_menu<'a>(
  options: &TerminalMenuOptions<'a>
) -> HashMap<&'a str, String> {
  println!();
  print_menu_header::print(
    options.header_text,
    &options.base_color,
    options.indent
  );
  let return_values = run_options::run(&options);
  println!();
  return return_values;
}
