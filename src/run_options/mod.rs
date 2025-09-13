use std::{ collections::HashMap };

use crate::definitions::{ InputEntry, TerminalMenuOptions };

pub fn run<'a>(options: &TerminalMenuOptions) -> HashMap<&'a str, &'a str> {
  let return_values: HashMap<&'a str, &'a str> = HashMap::new();
  options.input_entries.iter().for_each(|entry| {
    match entry {
      InputEntry::CHOOSABLE(entry) => {
        println!("{}", entry.text);
      }
      InputEntry::BOOL(entry) => {
        println!("{}", entry.text);
      }
    }
  });
  return return_values;
}
