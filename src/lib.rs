mod print_menu_header;
mod stylize;
pub mod colorize;

pub struct TerminalMenuOptions<'a> {
  base_color: colorize::ColorOptions,
  header_text: &'a str,
}

impl TerminalMenuOptions<'_> {
  pub fn new(
    header_text: &str,
    base_color: colorize::ColorOptions
  ) -> TerminalMenuOptions<'_> {
    TerminalMenuOptions {
      base_color,
      header_text,
    }
  }
}

pub fn run_terminal_menu(options: TerminalMenuOptions) {
  print_menu_header::print(options.header_text, options.base_color);
}
