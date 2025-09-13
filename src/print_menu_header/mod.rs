use colored::Styles;

use crate::{ colorize, stylize };

pub fn print(header_text: &str, base_color: colorize::ColorOptions) {
  let colorized_text = colorize::paint(header_text, base_color);
  let stylized_text = stylize::add_styles(
    colorized_text,
    vec![Styles::Bold, Styles::Underline]
  );
  println!("{}", stylized_text);
}
