use terminal_menu::{
  colorize::ColorOptions,
  run_terminal_menu,
  TerminalMenuOptions,
};

fn main() {
  run_terminal_menu(
    TerminalMenuOptions::new(ColorOptions::BLUE, 2, "Testing text")
  );
}
