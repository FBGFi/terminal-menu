use colored::{ Colorize, Styles };

use crate::{ colorize, definitions::ColorOptions, stylize, util };

pub fn print_header(header_text: &str, base_color: &ColorOptions, indent: u8) {
  let colorized_text = colorize::paint(header_text, base_color);
  let stylized_text = stylize::add_styles(
    colorized_text,
    vec![Styles::Bold, Styles::Underline]
  );
  util::print_line(stylized_text, indent + 2);
}

pub fn print_description(description: &Vec<&str>, indent: u8) {
  description.iter().for_each(|text| {
    util::print_line(text.white(), indent);
  });
}
