use terminal_menu::{
  definitions::{
    BooleanInputEntry,
    ChoosableInputEntry,
    ChoosableOption,
    InputEntry,
    TerminalMenuOptions,
    TextInputEntry,
  },
  run_terminal_menu,
};

fn main() {
  let options = TerminalMenuOptions {
    indent: 2,
    header_text: "Testing text",
    description: vec![
      "This is a description text,",
      "broken into multiple lines."
    ],
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
      }),
      InputEntry::TEXT(TextInputEntry {
        key: "text_entry",
        default: "default_value",
        text: "Testing text entry",
      })
    ],
  };
  let config = run_terminal_menu(&options, None);
  println!("{:?}", config);
}
