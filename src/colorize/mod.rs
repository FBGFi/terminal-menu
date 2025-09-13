use colored::{ ColoredString, Colorize };

use crate::definitions::ColorOptions;

pub fn paint(text: &str, color: &ColorOptions) -> ColoredString {
  return match color {
    ColorOptions::RED => { text.red() }
    ColorOptions::BLUE => { text.blue() }
  };
}
