use std::fmt::Display;

use crate::def::Line;

pub fn filter<T: Display + Clone>(lines: &[Line<T>]) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    lines
        .iter()
        .filter(|line| line.is_selected())
        .for_each(|line| result.push(line.clone().get_content()));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_nothing() {
        let line1 = Line::new(1);
        let line2 = Line::new(2);
        let lines = vec![line1, line2];
        let left = filter(&lines);
        let right: Vec<usize> = vec![];
        assert_eq!(left, right);
    }

    #[test]
    fn it_returns_one_value() {
        let mut line1 = Line::new(1);
        let line2 = Line::new(2);
        line1.toggle();
        let lines = vec![line1, line2];
        let left = filter(&lines);
        let right = vec![1];
        assert_eq!(left, right);
    }

    #[test]
    fn it_returns_two_values() {
        let mut line1 = Line::new("1");
        let mut line2 = Line::new("2");
        line1.toggle();
        line2.toggle();
        let lines = vec![line1, line2];
        let left = filter(&lines);
        let right: Vec<&'static str> = vec!["1", "2"];
        assert_eq!(left, right);
    }
}
