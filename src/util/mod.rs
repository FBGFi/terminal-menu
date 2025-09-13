use std::io::{ stdout, Write };

use colored::ColoredString;
use crossterm::cursor;
use terminal_size::{ terminal_size, Width };

fn get_n_spaces(num: u16) -> String {
  let mut spaces = String::new();
  for _ in 0..num {
    spaces.push(' ');
  }
  return spaces;
}

pub fn get_indents(indent: u8) -> String {
  return get_n_spaces(indent as u16);
}

pub fn print_line(text: ColoredString, indent: u8) {
  let text_indents = get_indents(indent);
  println!("{text_indents}{text}");
}

pub fn print(text: ColoredString, indent: u8) {
  let text_indents = get_indents(indent);
  print!("{text_indents}{text}");
  stdout().flush().ok().expect("failed to flush");
}

fn get_terminal_width() -> u16 {
  let size = terminal_size();
  match size {
    Some((Width(w), _)) => { w }
    None => {
      panic!("Terminal not supported");
    }
  }
}

pub fn get_current_cursor_row() -> u16 {
  let position = cursor::position();
  match position {
    Ok(pos) => pos.1,
    Err(_) => {
      panic!("Cursor position not found!");
    }
  }
}

pub fn clear_rest_of_row() {
  let cursor_position = cursor::position();
  let col = match cursor_position {
    Ok(pos) => pos.0,
    Err(_) => {
      panic!("Cursor position not found!");
    }
  };
  let terminal_width = get_terminal_width();
  let spaces = get_n_spaces(terminal_width - col);
  print!("{}", spaces);
  stdout().flush().ok().expect("failed to flush");
}
