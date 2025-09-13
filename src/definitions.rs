pub struct TerminalMenuOptions<'a> {
  pub base_color: ColorOptions,
  pub indent: u8,
  pub header_text: &'a str,
}

impl TerminalMenuOptions<'_> {
  pub fn new(
    base_color: ColorOptions,
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

pub enum ColorOptions {
  RED,
  BLUE,
}
