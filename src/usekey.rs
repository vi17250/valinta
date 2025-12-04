use crate::def::Line;
use console::Key;

pub fn use_key<T: std::fmt::Display>(lines: &mut [Line<T>], key_event: Key) {
    let length = lines.len();
    let current_index = lines
        .iter_mut()
        .position(|line| line.is_highlighted())
        .unwrap();

    match key_event {
        Key::ArrowDown => {
            lines[current_index].toggle_highlight();
            if current_index == length - 1 {
                lines[0].toggle_highlight();
            } else {
                lines[current_index + 1].toggle_highlight();
            }
        }
        Key::ArrowUp => {
            lines[current_index].toggle_highlight();
            if current_index == 0 {
                lines[length - 1].toggle_highlight();
            } else {
                lines[current_index - 1].toggle_highlight();
            }
        }
        Key::Char('a') | Key::Char('A') => lines.iter_mut().for_each(|line| line.check()),
        Key::Char('n') | Key::Char('N') => lines.iter_mut().for_each(|line| line.uncheck()),
        Key::Char('i') | Key::Char('I') => lines.iter_mut().for_each(|line| line.toggle_check()),
        Key::Char(' ') => lines[current_index].toggle_check(),
        _ => (),
    }
}
