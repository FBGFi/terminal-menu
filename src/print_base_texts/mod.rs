use colored::{ Colorize };

use crate::{ colorize, definitions::ColorOptions, util };

pub fn print_header(header_text: &str, base_color: &ColorOptions, indent: u8) {
  let colorized_text = colorize::paint(header_text, base_color);
  util::print_line(colorized_text.underline().bold(), indent + 2);
}

pub fn print_description(description: &Vec<&str>, indent: u8) {
  description.iter().for_each(|text| {
    util::print_line(text.white(), indent);
  });
}
