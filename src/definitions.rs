#[derive(Clone)]
pub enum ColorOptions {
  RED,
  BLUE,
  GREEN,
}

pub struct ChoosableOption<'a> {
  /// Text for the option
  pub text: &'a str,
  /// Value returned in hash map
  pub value: &'a str,
}

pub struct ChoosableInputEntry<'a> {
  /// Key for the returned hash map
  pub key: &'a str,
  /// Text to be shown as header
  pub text: &'a str,
  /// List of options
  pub options: Vec<ChoosableOption<'a>>,
}

pub struct BooleanInputEntry<'a> {
  /// Key for the returned hash map
  pub key: &'a str,
  /// Default if user input incorrect: true -> "y", false -> "n"
  pub default: bool,
  /// Text to be shown for entry
  pub text: &'a str,
}

pub struct TextInputEntry<'a> {
  /// Key for the returned hash map
  pub key: &'a str,
  /// Default if user input empty
  pub default: &'a str,
  /// Text to be shown for entry
  pub text: &'a str,
}

/// Enum of possible options to be passed for the module.
pub enum InputEntry<'a> {
  /// Takes in list of options that user can select from, returns selected option value
  CHOOSABLE(ChoosableInputEntry<'a>),
  /// Takes in user input and falls back to default for incorrect input, returns "y"|"n"
  BOOL(BooleanInputEntry<'a>),
  /// Takes in user input and falls back to default value for empty input
  TEXT(TextInputEntry<'a>),
}

/// Colors to be used for texts in terminal. Passing None uses following defaults:
/// * base_color: BLUE
/// * selected_option_color: GREEN
/// * falsy_selection_color: RED
#[derive(Clone)]
pub struct TerminalColors {
  /// Color that is used for majority of the texts.
  pub base_color: ColorOptions,
  /// Color that is used to print the option that user has selected.
  pub selected_option_color: ColorOptions,
  /// Color for falsy values (n in boolean selections).
  pub falsy_selection_color: ColorOptions,
}

/// Options for the module.
pub struct TerminalMenuOptions<'a> {
  /// Text indentation
  pub indent: u8,
  /// Header text for the menu
  pub header_text: &'a str,
  /// Description text for the menu, broken into multiple lines via vector cells
  pub description: Vec<&'a str>,
  /// List of options to configure
  pub input_entries: Vec<InputEntry<'a>>,
}
