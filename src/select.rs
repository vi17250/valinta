use console::{Key, Term};
use std::fmt::{Debug, Display};
use std::process;

use crate::def::Option;
use crate::display::display;
use crate::error::{Result, ValintaError};
use crate::filter::filter;

const NUMBER_TO_RENDER: u8 = 11;

pub fn select<T: Display + Debug + Clone>(items: &[T]) -> Result<Vec<T>> {
    if items.is_empty() {
        return Err(ValintaError::Custom("Input is empty".into()));
    }

    let mut options = items
        .iter()
        .enumerate()
        .map(|(index, thing)| Option::new(thing.clone(), index == 0))
        .collect::<Vec<Option<_>>>();

    let mut current_position = if options.len() < NUMBER_TO_RENDER as usize {
        0
    } else {
        options
            .iter()
            .position(|option| option.is_highlighted())
            .unwrap()
    };

    let raw = std::env::args_os().any(|arg| arg == "-r" || arg == "--raw");
    let term = Term::stdout();
    display(&options, &current_position);
    loop {
        let key = if raw {
            term.read_key_raw()
        } else {
            term.read_key()
        }?;
        match key {
            Key::ArrowUp => {
                if current_position > 0 {
                    current_position = current_position.saturating_sub(1);
                }
            }
            Key::ArrowDown => {
                if current_position < options.len() - 1 {
                    current_position = current_position.saturating_add(1);
                }
            }
            Key::Char(' ') => options
                .get_mut(current_position)
                .ok_or(ValintaError::Custom("Unexpected".into()))?
                .toggle(),
            Key::Enter => break,
            Key::Escape => process::exit(0),
            _ => (),
        }
        let _ = term.move_cursor_up(options.len());
        display(&options, &current_position);
    }

    let result = filter(&options);
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
