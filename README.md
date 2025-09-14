# @fbgfi/terminal-menu
Rust library that can be used for install wizards and like in the terminal.

## Usage
```rust
let config = run_terminal_menu!(
  TerminalMenuOptions {
    indent: 2,
    header_text: "This will be shown at the top of the menu",
    description: vec![
      "This is a description text,",
      "broken into multiple lines."
    ],
    input_entries: vec![
      InputEntry::BOOL(BooleanInputEntry {
        key: "boolean_entry",
        text: "This is text shown for entry",
        default: true, // true -> "y", false -> "n"
      }),
      InputEntry::CHOOSABLE(ChoosableInputEntry {
        key: "choosable_entry",
        text: "This is text shown on top of the list",
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
      InputEntry::TEXT(TextInputEntry {
        key: "text_entry",
        default: "default_value",
        text: "This is text shown for entry",
      })
    ],
  },
  // Optional, this is used as default value
  TerminalColors {
    base_color: ColorOptions::BLUE,
    selected_option_color: ColorOptions::GREEN,
    falsy_selection_color: ColorOptions::RED,
  }
);
```
