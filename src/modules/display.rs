use crate::modules::def::Line;

pub fn display<T: std::fmt::Display + Clone>(lines: &[Line<T>], current: &usize) {
    for (index, line) in lines.iter().enumerate() {
        if index == *current {
            println!("❯ {}", line);
        } else {
            println!("  {}", line);
        }
    }
}
