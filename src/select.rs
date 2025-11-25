use console::{Key, Term};
use std::fmt::{Debug, Display};
use std::process;

use crate::def::Option;
use crate::display::display;
use crate::error::{Result, ValintaError};
use crate::filter::filter;

pub fn select<T: Display + Debug + Clone>(items: &[T]) -> Result<Vec<T>> {
    if items.is_empty() {
        return Err(ValintaError::Custom("Input is empty".into()));
    }

    let mut options = items
        .iter()
        .enumerate()
        .map(|(index, thing)| Option::new(thing.clone(), index == 0))
        .collect::<Vec<Option<_>>>();

    let mut current: usize = 0;

    let raw = std::env::args_os().any(|arg| arg == "-r" || arg == "--raw");
    let term = Term::stdout();
    display(&options, &current);
    loop {
        let key = if raw {
            term.read_key_raw()
        } else {
            term.read_key()
        }?;
        match key {
            Key::ArrowUp => {
                if current > 0 {
                    current = current.saturating_sub(1);
                }
            }
            Key::ArrowDown => {
                if current < options.len() - 1 {
                    current = current.saturating_add(1);
                }
            }
            Key::Char(' ') => options
                .get_mut(current)
                .ok_or(ValintaError::Custom("Unexpected".into()))?
                .toggle(),
            Key::Enter => break,
            Key::Escape => process::exit(0),
            _ => (),
        }
        let _ = term.move_cursor_up(options.len());
        display(&options, &current);
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
