use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone)]
pub struct Line<T: Display + Clone> {
    content: T,
    is_selected: bool,
}

impl<T: Display + Clone> Line<T> {
    pub fn new(content: T) -> Self {
        Line {
            content,
            is_selected: false,
        }
    }
}

impl<T: Display + Clone> Line<T> {
    pub fn is_selected(&self) -> bool {
        self.is_selected
    }
}

impl<T: Display + Clone> Line<T> {
    pub fn toggle(&mut self) {
        self.is_selected = !self.is_selected
    }
}

impl<T: Display + Clone> Line<T> {
    pub fn get_content(self) -> T {
        self.content.clone()
    }
}

impl<T: Display + Clone> Display for Line<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let checked = String::from("🟢");
        let unchecked = String::from("⚪");
        let content = &self.content;
        if self.is_selected {
            write!(f, "{:<2} {}", checked, content)
        } else {
            write!(f, "{:<2} {}", unchecked, content)
        }
    }
}
