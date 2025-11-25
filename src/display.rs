use crate::def::Option;

pub fn display<T: std::fmt::Display + Clone>(lines: &[Option<T>], current: &usize) {
    for (index, line) in lines.iter().enumerate() {
        if index == *current {
            println!("❯ {}", line);
        } else {
            println!("  {}", line);
        }
    }
}
