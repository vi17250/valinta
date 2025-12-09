use crate::def::Line;
use console::style;

pub fn display<T: std::fmt::Display + Clone>(lines: &Vec<Line<T>>) {
    for line in lines {
        let render = if line.is_highlighted() {
            style(line).bold()
        } else {
            style(line)
        };
        println!("{}", render);
    }
}
