use terminal_menu::{
  colorize::ColorOptions,
  run_terminal_menu,
  TerminalMenuOptions,
};

fn main() {
  run_terminal_menu(
    TerminalMenuOptions::new("Testing text", ColorOptions::BLUE)
  );
}
