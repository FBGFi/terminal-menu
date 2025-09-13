use terminal_menu::{
  definitions::{ ColorOptions, TerminalMenuOptions },
  run_terminal_menu,
};

fn main() {
  run_terminal_menu(TerminalMenuOptions {
    base_color: ColorOptions::BLUE,
    indent: 2,
    header_text: "Testing text",
  });
}
