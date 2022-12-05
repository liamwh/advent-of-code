#![feature(iter_array_chunks)]
use std::collections::HashMap;

#[allow(clippy::almost_complete_letter_range)]
pub fn solve_part1(sack: &str) -> String {
  let item_type_priorities = ('a'..='z').chain('A'..='Z').into_iter().enumerate().map(|(i, c)| (c, i + 1)).collect::<HashMap<char, usize>>();

  let sum_of_item_type_priorities = sack.lines()
    .map(|line| {
    let compartment_length = line.len() / 2;
    let first_compartment = &line[0..compartment_length];
    let second_compartment = &line[compartment_length..];

    let common_char = first_compartment
      .chars()
      .find(|c| second_compartment.contains(*c))
      .unwrap();
    let common_char_priority = item_type_priorities.get(&common_char).unwrap();
    common_char_priority
  })
  .sum::<usize>();

  sum_of_item_type_priorities.to_string()
}

#[allow(unused_variables)]
pub fn solve_part2(sack: &str) -> String {
  let item_type_priorities = ('a'..='z').chain('A'..='Z').into_iter().enumerate().map(|(i, c)| (c, i + 1)).collect::<HashMap<char, usize>>();

  let sum_of_item_type_priorities = sack
    .lines()
    .array_chunks::<3>()
    .map(|[a,b,c]| {
      let first_compartment = a;
      let second_compartment = b;
      let third_compartment = c;

      let common_char = first_compartment
        .chars()
        .find(|c| second_compartment.contains(*c) && third_compartment.contains(*c))
        .unwrap();
      let common_char_priority = item_type_priorities.get(&common_char).unwrap();
      common_char_priority
    })
    .sum::<usize>();

  sum_of_item_type_priorities.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = solve_part1(PART_1_EXAMPLE_INPUT);
        assert_eq!(result, "157");
    }

    const PART_2_EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part2_works() {
        let result = solve_part2(PART_2_EXAMPLE_INPUT);
        assert_eq!(result, "70");
    }
}