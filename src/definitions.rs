#[derive(Clone)]
pub enum ColorOptions {
  RED,
  BLUE,
  GREEN,
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

/**
 * Colors to be used for texts in terminal. Passing None uses following defaults:
 * blue - majority of texts
 * red - falsy values
 * green - trueish values
 */
#[derive(Clone)]
pub struct TerminalColors {
  /**
   * Color that is used for majority of the texts.
   */
  pub base_color: ColorOptions,
  /**
   * Color that is used to print the option that user has selected.
   */
  pub selected_option_color: ColorOptions,
  /**
   * Color for falsy values (n in boolean selections).
   */
  pub falsy_selection_color: ColorOptions,
}

pub struct TerminalMenuOptions<'a> {
  pub indent: u8,
  pub header_text: &'a str,
  pub input_entries: Vec<InputEntry<'a>>,
}
