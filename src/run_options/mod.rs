use std::{ collections::HashMap, io::{ stdin, stdout, Write } };
use colored::{ ColoredString, Colorize };
use crossterm::{
  cursor,
  event::{ read, Event, KeyCode, KeyEvent, KeyEventKind },
  queue,
  terminal::{ disable_raw_mode, enable_raw_mode },
};

use crate::{
  colorize,
  definitions::{ InputEntry, TerminalColors, TerminalMenuOptions },
  util::{ self, clear_rest_of_row, get_current_cursor_row },
};

fn format_bool_options_text(default_value: bool) -> String {
  return match default_value {
    true => format!("({},n)", "Y".underline()),
    false => format!("(y,{})", "N".underline()),
  };
}

fn bool_input_to_text(
  bool_input: &String,
  terminal_colors: &TerminalColors
) -> ColoredString {
  return match bool_input.as_str() {
    "y" => colorize::paint("Yes", &terminal_colors.selected_option_color),
    "n" => colorize::paint("No", &terminal_colors.falsy_selection_color),
    _ => {
      panic!("Unknown boolean input: {}", bool_input);
    }
  };
}

fn get_bool_input(input: String, default_value: bool) -> String {
  let first_char = input.chars().nth(0);
  return match first_char {
    Some('y') => "y".to_string(),
    Some('n') => "n".to_string(),
    _ => {
      match default_value {
        true => "y".to_string(),
        false => "n".to_string(),
      }
    }
  };
}

pub fn run<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: &TerminalColors
) -> HashMap<&'a str, String> {
  let mut return_values: HashMap<&'a str, String> = HashMap::new();
  options.input_entries.iter().for_each(|entry| {
    match entry {
      InputEntry::BOOL(entry) => {
        let text = format!(
          "{} {}: ",
          entry.text,
          format_bool_options_text(entry.default)
        );
        util::print(text.white(), options.indent);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Did not enter correct string");
        let bool_input = get_bool_input(input, entry.default);
        let updated_text = format!(
          "{}: {}",
          entry.text,
          bool_input_to_text(&bool_input, terminal_colors)
        );
        queue!(
          stdout(),
          cursor::MoveTo(0, get_current_cursor_row() - 1)
        ).unwrap();
        util::print(
          colorize::paint(updated_text.as_str(), &terminal_colors.base_color),
          options.indent
        );
        clear_rest_of_row();
        println!();
        return_values.insert(entry.key, bool_input);
      }
      InputEntry::CHOOSABLE(entry) => {
        enable_raw_mode().unwrap();
        queue!(stdout(), cursor::Hide).unwrap();
        let text = format!("{} (choose an option): ", entry.text);
        util::print(
          colorize::paint(
            text.as_str(),
            &terminal_colors.falsy_selection_color
          ),
          options.indent
        );
        println!();
        let cursor_start_position = get_current_cursor_row();
        let mut cursor_current_position = cursor_start_position;
        let mut current_option_index = (cursor_current_position -
          cursor_start_position) as usize;
        let mut chose_option = false;
        while !chose_option {
          for (i, option) in entry.options.iter().enumerate() {
            queue!(
              stdout(),
              cursor::MoveTo(0, cursor_start_position + (i as u16))
            ).unwrap();
            let list_text = format!("• {}", option.text);
            if i == current_option_index {
              util::print_line(
                colorize::paint(
                  &list_text,
                  &terminal_colors.selected_option_color
                ),
                options.indent
              );
            } else {
              util::print_line(list_text.white(), options.indent);
            }
          }
          queue!(stdout(), cursor::MoveTo(0, cursor_current_position)).unwrap();
          stdout().flush().ok().expect("failed to flush");
          match read().unwrap() {
            Event::Key(
              KeyEvent { code: KeyCode::Up, kind: KeyEventKind::Press, .. },
            ) => {
              if current_option_index != 0 {
                current_option_index = current_option_index - 1;
                cursor_current_position = cursor_current_position - 1;
              }
            }
            Event::Key(
              KeyEvent { code: KeyCode::Down, kind: KeyEventKind::Press, .. },
            ) => {
              if current_option_index != entry.options.len() - 1 {
                current_option_index = current_option_index + 1;
                cursor_current_position = cursor_current_position + 1;
              }
            }
            Event::Key(
              KeyEvent { code: KeyCode::Enter, kind: KeyEventKind::Press, .. },
            ) => {
              return_values.insert(
                entry.key,
                entry.options
                  .get(current_option_index)
                  .unwrap()
                  .value.to_string()
              );
              chose_option = true;
            }
            _ => {
              chose_option = false;
            }
          }
        }
        for i in 0..entry.options.len() {
          queue!(
            stdout(),
            cursor::MoveTo(0, cursor_start_position + (i as u16))
          ).unwrap();
          clear_rest_of_row();
        }
        queue!(stdout(), cursor::MoveTo(0, cursor_start_position - 1)).unwrap();
        let updated_text = format!(
          "{}: {}",
          colorize::paint(entry.text, &terminal_colors.base_color),
          colorize::paint(
            entry.options.get(current_option_index).unwrap().text,
            &terminal_colors.selected_option_color
          )
        );
        util::print(updated_text, options.indent);
        clear_rest_of_row();
        queue!(stdout(), cursor::MoveTo(0, cursor_start_position)).unwrap();
        disable_raw_mode().unwrap();
        queue!(stdout(), cursor::Show).unwrap();
      }
    }
  });
  return return_values;
}
