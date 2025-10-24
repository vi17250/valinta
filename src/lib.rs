pub mod multi_select {
    use console::{Key, Term};
    use std::fmt::{Debug, Display};
    use std::process;

    mod def;
    use def::Line;
    pub mod error;
    use error::Result;

    use crate::multi_select::error::ValintaError::Custom;

    mod filter;
    use crate::multi_select::filter::filter;

    mod display;
    use crate::multi_select::display::display;

    pub fn selection<T: Display + Debug + Clone>(things: &[T]) -> Result<Vec<T>> {
        if things.is_empty() {
            return Err(Custom("Input is empty".into()));
        }

        let mut lines = things
            .iter()
            .map(|thing| Line::new(thing.clone()))
            .collect::<Vec<Line<_>>>();

        let mut current: usize = 0;

        let raw = std::env::args_os().any(|arg| arg == "-r" || arg == "--raw");
        let term = Term::stdout();
        display(&lines, &current);
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
                    if current < lines.len() - 1 {
                        current = current.saturating_add(1);
                    }
                }
                Key::Char(' ') => lines
                    .get_mut(current)
                    .ok_or(Custom("Unexpected".into()))?
                    .toggle(),
                Key::Enter => break,
                Key::Escape => process::exit(0),
                _ => (),
            }
            let _ = term.move_cursor_up(lines.len());
            display(&lines, &current);
        }
        let result = filter(&lines);
        Ok(result)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_throws_err() {
            let input: Vec<i32> = Vec::new();
            let result = selection(&input);
            assert!(result.is_err());
        }
    }
}
