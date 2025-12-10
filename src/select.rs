use console::{Key, Term};
use std::fmt::Display;
use std::process;

use crate::def::Line;
use crate::error::{Result, ValintaError};
use crate::terminal::{display, get_width};
use crate::usekey::use_key;
use crate::utils::{array, number_of_rendered_lines};

pub type Returned<T> = (Vec<T>, Vec<usize>);

const NUMBER_TO_RENDER: usize = 11;

pub fn select<T: Display + Clone>(items: &[T]) -> Result<Returned<T>> {
    if items.is_empty() {
        return Err(ValintaError::Custom("Input is empty".into()));
    }

    let mut lines = items
        .iter()
        .enumerate()
        .map(|(index, option)| Line::new(option, index == 0))
        .collect::<Vec<Line<&T>>>();


    let mut cursor_position = 0;

    let mut rendered_lines = array(&lines, Some(cursor_position), Some(NUMBER_TO_RENDER));

    display(&rendered_lines);

    let raw = std::env::args_os().any(|arg| arg == "-r" || arg == "--raw");
    let term = Term::stdout();
    loop {
        let key = if raw {
            term.read_key_raw()
        } else {
            term.read_key()
        }
        .unwrap();

        match key {
            Key::Enter => break,
            Key::Escape => process::exit(0),
            key_pressed => use_key(&mut lines, key_pressed),
        }

        let terminal_width = get_width()?;

        let number_of_rendered_lines = number_of_rendered_lines(&rendered_lines, terminal_width);

        let _ = term.move_cursor_up(number_of_rendered_lines);
        let _ = term.clear_to_end_of_screen();

        cursor_position = if lines.len() < NUMBER_TO_RENDER {
            0
        } else {
            lines.iter().position(|line| line.is_highlighted()).unwrap()
        };

        rendered_lines = array(&lines, Some(cursor_position), Some(NUMBER_TO_RENDER));

        display(&rendered_lines);
    }

    let result: Returned<T> =
        lines
            .iter()
            .enumerate()
            .fold((vec![], vec![]), |acc, (index, line)| {
                if line.is_checked() {
                    let content = line.get_content().clone();
                    let mut t1 = acc.0;
                    let mut t2 = acc.1;
                    t1.push(content);
                    t2.push(index);
                    (t1, t2)
                } else {
                    acc
                }
            });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_throws_err() {
        let input: Vec<i32> = Vec::new();
        let result = select(&input);
        assert!(result.is_err());
    }
}
