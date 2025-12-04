use crate::def::Line;

pub fn display<T: std::fmt::Display + Clone>(lines: &Vec<Line<T>>) {
    for line in lines {
            println!("{}", line);
    }
}
