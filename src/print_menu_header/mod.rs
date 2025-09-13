use colored::Styles;

use crate::{ colorize, stylize, util };

pub fn print(
  header_text: &str,
  base_color: colorize::ColorOptions,
  indent: u8
) {
  let colorized_text = colorize::paint(header_text, base_color);
  let stylized_text = stylize::add_styles(
    colorized_text,
    vec![Styles::Bold, Styles::Underline]
  );
  util::print_line(stylized_text, indent);
}
