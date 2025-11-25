use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone)]
pub struct Option<T: Display + Clone> {
    checked: bool,
    content: T,
    highlighted: bool,
}

impl<T: Display + Clone> Option<T> {
    pub fn new(content: T, highlighted: bool) -> Self {
        Option {
            checked: false,
            content,
            highlighted,
        }
    }
}

impl<T: Display + Clone> Option<T> {
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

impl<T: Display + Clone> Option<T> {
    pub fn is_highlighted(&self) -> bool {
        self.checked
    }
}

impl<T: Display + Clone> Option<T> {
    pub fn toggle(&mut self) {
        self.checked = !self.checked
    }
}

impl<T: Display + Clone> Option<T> {
    pub fn get_content(self) -> T {
        self.content.clone()
    }
}

impl<T: Display + Clone> Display for Option<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let checked = String::from("[x]");
        let unchecked = String::from("[ ]");
        let content = &self.content;
        if self.checked {
            write!(f, "{:<2} {}", checked, content)
        } else {
            write!(f, "{:<2} {}", unchecked, content)
        }
    }
}
