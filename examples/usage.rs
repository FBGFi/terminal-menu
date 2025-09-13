use terminal_menu::{
  definitions::{
    BooleanInputEntry,
    ChoosableInputEntry,
    ChoosableOption,
    ColorOptions,
    InputEntry,
    TerminalMenuOptions,
  },
  run_terminal_menu,
};

fn main() {
  let options = TerminalMenuOptions {
    base_color: ColorOptions::BLUE,
    indent: 2,
    header_text: "Testing text",
    input_entries: vec![
      InputEntry::BOOL(BooleanInputEntry {
        key: "boolean_entry",
        text: "Testing boolean entry",
        default: true,
      }),
      InputEntry::CHOOSABLE(ChoosableInputEntry {
        key: "choosable_entry",
        text: "Testing entry with multiple choices",
        options: vec![
          ChoosableOption {
            text: "Option 1",
            value: "option_1",
          },
          ChoosableOption {
            text: "Option 2",
            value: "option_2",
          }
        ],
      }),
      InputEntry::BOOL(BooleanInputEntry {
        key: "another_boolean_entry",
        text: "Testing another boolean entry",
        default: false,
      })
    ],
  };
  let config = run_terminal_menu(&options);
  println!("{:?}", config);
}
