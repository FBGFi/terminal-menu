use std::{ collections::HashMap, io::{ stdout, Write }, process::exit };

use colored::Colorize;
use crossterm::{
  cursor,
  event::{ read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers },
  queue,
  terminal::{ disable_raw_mode, enable_raw_mode },
};

use crate::{
  colorize,
  definitions::{ ChoosableInputEntry, TerminalColors, TerminalMenuOptions },
  util::{
    self,
    clear_rest_of_row,
    get_current_cursor_row,
    get_terminal_height,
  },
};

pub fn run_choosable_input<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: &TerminalColors,
  return_values: &mut HashMap<&'a str, String>,
  entry: &ChoosableInputEntry<'a>
) {
  enable_raw_mode().unwrap();
  queue!(stdout(), cursor::Hide).unwrap();
  let text = format!("{} (choose an option): ", entry.text);
  util::print(
    colorize::paint(text.as_str(), &terminal_colors.falsy_selection_color),
    options.indent
  );
  println!();
  let cursor_start_position = get_current_cursor_row();
  let terminal_height = get_terminal_height();

  let mut cursor_current_position = cursor_start_position;
  let mut current_option_index: usize = 0;
  let mut chose_option = false;
  while !chose_option {
    for (i, option) in entry.options.iter().enumerate() {
      queue!(
        stdout(),
        cursor::MoveTo(0, cursor_current_position + (i as u16))
      ).unwrap();
      let list_text = format!("• {}", option.text);
      if i == current_option_index {
        util::print_line(
          colorize::paint(&list_text, &terminal_colors.selected_option_color),
          options.indent
        );
      } else {
        util::print_line(list_text.white(), options.indent);
      }
    }
    if cursor_start_position >= terminal_height - 1 {
      cursor_current_position =
        cursor_start_position - (entry.options.len() as u16);
    }
    stdout().flush().ok().expect("failed to flush");
    match read().unwrap() {
      Event::Key(
        KeyEvent { code: KeyCode::Up, kind: KeyEventKind::Press, .. },
      ) => {
        if current_option_index != 0 {
          current_option_index = current_option_index - 1;
        }
      }
      Event::Key(
        KeyEvent { code: KeyCode::Down, kind: KeyEventKind::Press, .. },
      ) => {
        if current_option_index != entry.options.len() - 1 {
          current_option_index = current_option_index + 1;
        }
      }
      Event::Key(
        KeyEvent { code: KeyCode::Enter, kind: KeyEventKind::Press, .. },
      ) => {
        return_values.insert(
          entry.key,
          entry.options.get(current_option_index).unwrap().value.to_string()
        );
        chose_option = true;
      }
      Event::Key(
        KeyEvent {
          code: KeyCode::Char('c'),
          kind: KeyEventKind::Press,
          modifiers: KeyModifiers::CONTROL,
          ..
        },
      ) => {
        queue!(stdout(), cursor::Show).unwrap();
        queue!(
          stdout(),
          cursor::MoveTo(
            0,
            cursor_start_position + (entry.options.len() as u16) + 1
          )
        ).unwrap();
        disable_raw_mode().unwrap();
        println!();
        println!("Exited program.");
        println!();
        exit(0);
      }
      _ => {
        chose_option = false;
      }
    }
  }
  for i in 0..entry.options.len() {
    queue!(
      stdout(),
      cursor::MoveTo(0, cursor_current_position + (i as u16))
    ).unwrap();
    clear_rest_of_row();
  }
  queue!(stdout(), cursor::MoveTo(0, cursor_current_position - 1)).unwrap();
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
  queue!(stdout(), cursor::MoveTo(0, cursor_current_position)).unwrap();
  disable_raw_mode().unwrap();
  queue!(stdout(), cursor::Show).unwrap();
}
