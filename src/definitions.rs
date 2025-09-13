pub enum ColorOptions {
  RED,
  BLUE,
}

pub struct ChoosableOption<'a> {
  pub text: &'a str,
  pub value: &'a str,
}

pub struct ChoosableInputEntry<'a> {
  pub key: &'a str,
  pub text: &'a str,
  pub options: Vec<ChoosableOption<'a>>,
}

pub struct BooleanInputEntry<'a> {
  pub key: &'a str,
  pub default: bool,
  pub text: &'a str,
}

pub enum InputEntry<'a> {
  CHOOSABLE(ChoosableInputEntry<'a>),
  BOOL(BooleanInputEntry<'a>),
}

pub struct TerminalMenuOptions<'a> {
  pub base_color: ColorOptions,
  pub indent: u8,
  pub header_text: &'a str,
  pub input_entries: Vec<InputEntry<'a>>,
}
