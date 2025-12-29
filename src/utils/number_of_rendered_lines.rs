//! Returns the number of lines that will be needed to display the content

use unicode_width::UnicodeWidthStr;

pub fn number_of_rendered_lines<T: std::fmt::Display>(
    rendered_lines: &[T],
    terminal_width: u16,
) -> usize {
    rendered_lines.iter().fold(0, |acc, option| {
        let current_lines = number_of_lines(option, terminal_width);
        acc + current_lines
    })
}

fn number_of_lines<T: std::fmt::Display>(content: &T, max_width: u16) -> usize {
    let value = content.to_string();
    value.split("\n").fold(0, |acc, val| {
        let len = val.width() / max_width as usize + 1;
        acc + len
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_one_and_one() {
        // Given
        let content = vec!["Line 1","Line 2"];

        // When
        let lines = number_of_rendered_lines(&content, 16);

        // Then
        assert_eq!(lines, 2);
    }

    #[test]
    fn it_returns_one_and_two() {
        // Given
        let content = vec!["Line 1","First line\nOther line"];

        // When
        let lines = number_of_rendered_lines(&content, 16);

        // Then
        assert_eq!(lines, 3);
    }

    #[test]
    fn it_returns_one_and_four() {
        // Given
        let content = ["One line", "14 char length\n14 char length"];

        // When
        let lines = number_of_rendered_lines(&content, 10);

        // Then
        assert_eq!(lines, 5);
    }

    #[test]
    fn it_returns_lines_when_inputs_are_integer() {
        // Given
        let content = vec![10,1000];

        // When
        let lines = number_of_rendered_lines(&content, 3);

        // Then
        assert_eq!(lines, 3);
    }

    #[test]
    fn it_returns_one() {
        // Given
        let content = "A blue world 🌍";

        // When
        let lines = number_of_lines(&content, 16);

        // Then
        assert_eq!(lines, 1);
    }

    #[test]
    fn it_returns_two() {
        // Given
        let content = "First line\nOther line";

        // When
        let lines = number_of_lines(&content, 16);

        // Then
        assert_eq!(lines, 2);
    }

    #[test]
    fn it_returns_four() {
        // Given
        let content = "14 char length\n14 char length";

        // When
        let lines = number_of_lines(&content, 10);

        // Then
        assert_eq!(lines, 4);
    }
    #[test]
    fn it_returns_lines_when_input_is_integer() {
        // Given
        let content = 1000;

        // When
        let lines = number_of_lines(&content, 3);

        // Then
        assert_eq!(lines, 2);
    }
}
