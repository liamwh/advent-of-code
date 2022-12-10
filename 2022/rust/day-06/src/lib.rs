// Returns the index of the first character where the window of size `size` is unique,
// and the window itself.
fn find_unique_window(chars: Vec<char>, window_size: usize) -> Option<(usize, Vec<char>)> {
    let sequence = chars.windows(window_size).enumerate().find(|(_i, window)| {
        let set = window.iter().collect::<std::collections::HashSet<&char>>();
        window.len() == set.len()
    });
    sequence.map(|sequence| (sequence.0 + window_size - 1, sequence.1.to_vec()))
}

pub fn solve_part1(datastream_buffer: &str) -> String {
    let chars = datastream_buffer.chars().collect::<Vec<char>>();
    let sequence = find_unique_window(chars, 4).unwrap();
    (sequence.0 + 1).to_string()
}

pub fn solve_part2(datastream_buffer: &str) -> String {
    let chars = datastream_buffer.chars().collect::<Vec<char>>();
    let sequence = find_unique_window(chars, 14).unwrap();
    (sequence.0 + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_unique_window_works_with_example_string1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = find_unique_window(input.chars().collect(), 4);
        assert_eq!(result, Some((6, vec!['j', 'p', 'q', 'm'])));
    }

    #[test]
    fn find_unique_window_with_too_short_input_returns_none() {
        let input = "mjq";
        let result = find_unique_window(input.chars().collect(), 4);
        assert_eq!(result, None);
    }

    #[test]
    fn find_unique_window_with_no_unique_values_returns_none() {
        let input = "mjjj";
        let result = find_unique_window(input.chars().collect(), 4);
        assert_eq!(result, None);
    }

    #[test]
    fn part1_works_with_example_string1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = solve_part1(input);
        assert_eq!(result, "7");
    }

    #[test]
    fn part1_works_with_example_string2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = solve_part1(input);
        assert_eq!(result, "5");
    }

    #[test]
    fn part2_works_with_example_string2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = solve_part2(input);
        assert_eq!(result, "19");
    }
}
