mod print_menu_header;
mod stylize;
mod util;
pub mod colorize;

pub struct TerminalMenuOptions<'a> {
  base_color: colorize::ColorOptions,
  indent: u8,
  header_text: &'a str,
}

impl TerminalMenuOptions<'_> {
  pub fn new(
    base_color: colorize::ColorOptions,
    indent: u8,
    header_text: &str
  ) -> TerminalMenuOptions<'_> {
    TerminalMenuOptions {
      base_color,
      indent,
      header_text,
    }
  }
}

pub fn run_terminal_menu(options: TerminalMenuOptions) {
  println!();
  print_menu_header::print(
    options.header_text,
    options.base_color,
    options.indent
  );
  println!();
}
