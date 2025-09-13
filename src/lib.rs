use crate::definitions::TerminalMenuOptions;

mod print_menu_header;
mod stylize;
mod util;
mod colorize;
pub mod definitions;

pub fn run_terminal_menu(options: TerminalMenuOptions) {
  println!();
  print_menu_header::print(
    options.header_text,
    options.base_color,
    options.indent
  );
  println!();
}
