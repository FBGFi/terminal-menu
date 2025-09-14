use colored::{ ColoredString, Colorize };

use crate::definitions::ColorOptions;

/// Sets text color based on given options.
pub fn paint(text: &str, color: &ColorOptions) -> ColoredString {
  return match color {
    ColorOptions::RED => { text.red() }
    ColorOptions::BLUE => { text.blue() }
    ColorOptions::GREEN => { text.green() }
  };
}
