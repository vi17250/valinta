//! Returns the original array but offsetted by the `offset param`,
//! It's possible to slice the result

pub fn array<T: Copy + Clone>(values: &[T], offset: Option<usize>, length: Option<usize>) -> Vec<T> {
    let values_len = values.len();
    let offset_vector = match offset {
        Some(offset) if offset <= values_len => values
            .iter()
            .enumerate()
            .map(|(index, _)| {
                if offset + index < values_len {
                    values[offset + index]
                } else {
                    let delta = offset + index - values_len;
                    values[delta]
                }
            })
            .collect::<Vec<T>>(),
        _ => values.to_vec(),
    };

    match length {
        Some(length) if length <= values_len => offset_vector[..length].to_vec(),
        _ => offset_vector,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::def::Line;

    #[test]
    fn it_returns_same_array() {
        // Given
        let l1 = Line::new("content1", false);
        let l2 = Line::new("content2", false);
        let input = vec![l1,l2];

        // When
        let offsetted = array(&input, None, None);

        // Then
        assert_eq!(
            offsetted,
            vec![Line::new("content1", false), Line::new("content2", false)]
        );
    }

    #[test]
    fn it_offset_array_from_one() {
        // Given
        let input = vec![&1, &2, &3];

        // When
        let offsetted = array(&input, Some(1), None);

        // Then
        assert_eq!(offsetted, vec![&2, &3, &1]);
    }

    #[test]
    fn it_offset_array_from_two() {
        // Given
        let input = vec!["a", "b", "c", "d"];

        // When
        let offsetted = array(&input, Some(2), None);

        // Then
        assert_eq!(offsetted, vec!["c", "d", "a", "b"]);
    }

    #[test]
    fn it_offset_array_from_three() {
        // Given
        let input = vec!["a", "b", "c", "d"];

        // When
        let offsetted = array(&input, Some(3), None);

        // Then
        assert_eq!(offsetted, vec!["d", "a", "b", "c"]);
    }

    #[test]
    fn it_returns_the_first_two_elements() {
        // Given
        let input = vec!["a", "b", "c", "d"];

        // When
        let offsetted = array(&input, None, Some(2));

        // Then
        assert_eq!(offsetted, vec!["a", "b"]);
    }

    #[test]
    fn it_split_and_offset_array_from_three() {
        // Given
        let input = vec!["a", "b", "c", "d"];

        // When
        let offsetted = array(&input, Some(3), Some(2));

        // Then
        assert_eq!(offsetted, vec!["d", "a"]);
    }

    #[test]
    fn it_returns_same_array_offset_out_of_bounds() {
        // Given
        let input = vec!["a", "b", "c", "d"];

        // When
        let offsetted = array(&input, Some(10), None);

        // Then
        assert_eq!(offsetted, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn it_returns_same_array_length_out_of_bounds() {
        // Given
        let input = vec!["a", "b", "c", "d"];

        // When
        let offsetted = array(&input, None, Some(10));

        // Then
        assert_eq!(offsetted, vec!["a", "b", "c", "d"]);
    }
}
