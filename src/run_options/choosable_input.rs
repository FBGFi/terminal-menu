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

/// Hides cursor and moves terminal to raw mode to prevent key presses to be always printed.
fn enter_input_mode() {
  queue!(stdout(), cursor::Hide).unwrap();
  enable_raw_mode().unwrap();
}

/// Resets terminal to original state.
fn exit_input_mode() {
  disable_raw_mode().unwrap();
  queue!(stdout(), cursor::Show).unwrap();
}

/// If user pressed ctrl+c while in input mode.
fn handle_user_exit() {
  exit_input_mode();
  println!();
  println!("Exited program.");
  println!();
  exit(0);
}

/// Prints option header.
fn print_header_text<'a>(
  text: &'a str,
  terminal_colors: &TerminalColors,
  indent: u8
) {
  let text = format!("{} (choose an option): ", text);
  util::print(
    colorize::paint(text.as_str(), &terminal_colors.falsy_selection_color),
    indent
  );
  // Move cursor to new row
  println!();
}

/// Prints list of options, coloring the current selection.
fn print_options_list<'a>(
  terminal_colors: &TerminalColors,
  entry: &ChoosableInputEntry<'a>,
  indent: u8,
  cursor_current_position: u16,
  current_option_index: usize
) {
  for (i, option) in entry.options.iter().enumerate() {
    queue!(
      stdout(),
      cursor::MoveTo(0, cursor_current_position + (i as u16))
    ).unwrap();
    let list_text = format!("• {}", option.text);
    if i == current_option_index {
      util::print_line(
        colorize::paint(&list_text, &terminal_colors.selected_option_color),
        indent
      );
    } else {
      util::print_line(list_text.white(), indent);
    }
  }
  stdout().flush().ok().expect("failed to flush");
}

/// Runs a loop for reading user key presses.
///
/// Up and down arrows are used to navigating between options, enter is used for selecting
/// option and ctrl+c will exit the program.
fn run_input_mode<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: &TerminalColors,
  return_values: &mut HashMap<&'a str, String>,
  entry: &ChoosableInputEntry<'a>
) -> (u16, usize) {
  let cursor_start_position = get_current_cursor_row();
  let terminal_height = get_terminal_height();

  let mut cursor_current_position = cursor_start_position;
  let mut current_option_index: usize = 0;

  print_options_list(
    terminal_colors,
    entry,
    options.indent,
    cursor_current_position,
    current_option_index
  );

  // If we are at the bottom of terminal, we need to move cursor up for
  // prints to correctly overwrite previous entries
  if cursor_start_position >= terminal_height - 1 {
    cursor_current_position =
      cursor_start_position - (entry.options.len() as u16);
  }

  loop {
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
        break;
      }
      Event::Key(
        KeyEvent {
          code: KeyCode::Char('c'),
          kind: KeyEventKind::Press,
          modifiers: KeyModifiers::CONTROL,
          ..
        },
      ) => {
        handle_user_exit();
      }
      _ => {
        continue;
      }
    }

    // Overwrite list based on current selection
    print_options_list(
      terminal_colors,
      entry,
      options.indent,
      cursor_current_position,
      current_option_index
    );
  }

  return (cursor_current_position, current_option_index);
}

/// Clears options list from terminal.
fn clear_options<'a>(
  entry: &ChoosableInputEntry<'a>,
  cursor_final_position: u16
) {
  for i in 0..entry.options.len() {
    queue!(
      stdout(),
      cursor::MoveTo(0, cursor_final_position + (i as u16))
    ).unwrap();
    clear_rest_of_row();
  }
  queue!(stdout(), cursor::MoveTo(0, cursor_final_position - 1)).unwrap();
}

/// Print formatted text to be shown after input mode is to be exited.
fn print_final_text<'a>(
  entry: &ChoosableInputEntry<'a>,
  terminal_colors: &TerminalColors,
  indent: u8,
  cursor_final_position: u16,
  selected_option_index: usize
) {
  let updated_text = format!(
    "{}: {}",
    colorize::paint(entry.text, &terminal_colors.base_color),
    colorize::paint(
      entry.options.get(selected_option_index).unwrap().text,
      &terminal_colors.selected_option_color
    )
  );
  util::print(updated_text, indent);
  clear_rest_of_row();
  queue!(stdout(), cursor::MoveTo(0, cursor_final_position)).unwrap();
}

/// Function that sets terminal to raw mode for arrow navigation between passed options.
///
/// Inserted value will be key value pair of (key, option.value).
pub fn run_choosable_input<'a>(
  options: &TerminalMenuOptions<'a>,
  terminal_colors: &TerminalColors,
  return_values: &mut HashMap<&'a str, String>,
  entry: &ChoosableInputEntry<'a>
) {
  enter_input_mode();
  print_header_text(entry.text, terminal_colors, options.indent);

  let (cursor_final_position, selected_option_index) = run_input_mode(
    options,
    terminal_colors,
    return_values,
    entry
  );

  clear_options(entry, cursor_final_position);
  print_final_text(
    entry,
    terminal_colors,
    options.indent,
    cursor_final_position,
    selected_option_index
  );
  exit_input_mode();
}
