use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Line<T: Display> {
    checked: bool,
    content: T,
    highlighted: bool,
}

impl<T: Display> Line<T> {
    pub fn new(content: T, highlighted: bool) -> Self {
        Line {
            checked: false,
            content,
            highlighted,
        }
    }
}

impl<T: Display> Line<T> {
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

impl<T: Display> Line<T> {
    pub fn is_highlighted(&self) -> bool {
        self.highlighted
    }
}

impl<T: Display> Line<T> {
    pub fn toggle_highlight(&mut self)  {
        self.highlighted = !self.highlighted
    }
}

impl<T: Display> Line<T> {
    pub fn toggle_check(&mut self) {
        self.checked = !self.checked
    }
}

impl<T: Display> Line<T> {
    pub fn check(&mut self) {
        self.checked = true
    }
}
impl<T: Display> Line<T> {
    pub fn uncheck(&mut self) {
        self.checked = false
    }
}

impl<T: Display + Clone> Line<T> {
    pub fn get_content(self) -> T {
        self.content.clone()
    }
}

impl<T: Display> Display for Line<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let checkbox = if self.is_checked() { "[x]" } else { "[ ]" };
        write!(f, "{} {}", checkbox, self.content)
    }
}
