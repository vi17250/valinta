use crate::def::Line;
use console::style;

pub fn display<T: std::fmt::Display + Clone>(lines: &Vec<Line<T>>) {
    for line in lines {
        let formatted = format!(" {}", line);
        let render = if line.is_highlighted() {
            let icon = style("❯").color256(81);
            format!("{}{}", icon, formatted)
        } else {
            format!(" {}", formatted)
        };
        println!("{}", render);
    }
}
