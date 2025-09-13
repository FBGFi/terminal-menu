use colored::{ ColoredString, Colorize };

pub enum ColorOptions {
  RED,
  BLUE,
}

pub fn paint(text: &str, color: ColorOptions) -> ColoredString {
  return match color {
    ColorOptions::RED => { text.red() }
    ColorOptions::BLUE => { text.blue() }
  };
}
