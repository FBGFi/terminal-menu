use colored::ColoredString;

pub fn get_indents(indent: u8) -> String {
  let mut indent_string = String::new();
  for _ in 0..indent {
    indent_string.push(' ');
  }
  return indent_string;
}

pub fn print_line(text: ColoredString, indent: u8) {
  let text_indents = get_indents(indent);
  println!("{text_indents}{text}");
}
