#[allow(unused_variables)]
pub fn solve_part1(list_of_section_assignments_for_each_pair_of_elves: &str) -> String {
  let result: u32 = list_of_section_assignments_for_each_pair_of_elves
    .lines()
    .map(|section_pairs| {
      let assigned_section_ids_range = section_pairs
      .split(',')
      .map(|section| {
        let section_ids: Vec<&str> = section
          .split('-').collect();
        let assigned_section_ids_for_this_elf: Vec<u32> = (section_ids[0].parse::<u32>().unwrap()..section_ids[1].parse::<u32>().unwrap() + 1).collect();
        assigned_section_ids_for_this_elf
      }
      ).collect::<Vec<Vec<u32>>>();
      let intersection = assigned_section_ids_range[0].iter().cloned().filter(|x| assigned_section_ids_range[1].contains(x)).collect::<Vec<u32>>();
      if intersection == assigned_section_ids_range[0] || intersection == assigned_section_ids_range[1] {
        1
      } else {
        0
      }
    })
    .sum::<u32>();

  result.to_string()
}

#[allow(unused_variables)]
pub fn solve_part2(list_of_section_assignments_for_each_pair_of_elves: &str) -> String {
  let result: u32 = list_of_section_assignments_for_each_pair_of_elves
    .lines()
    .map(|section_pairs| {
      let assigned_section_ids_range = section_pairs
      .split(',')
      .map(|section| {
        let section_ids: Vec<&str> = section
          .split('-').collect();
        let assigned_section_ids_for_this_elf: Vec<u32> = (section_ids[0].parse::<u32>().unwrap()..section_ids[1].parse::<u32>().unwrap() + 1).collect();
        assigned_section_ids_for_this_elf
      }
      ).collect::<Vec<Vec<u32>>>();
      let intersection = assigned_section_ids_range[0].iter().cloned().filter(|x| assigned_section_ids_range[1].contains(x)).collect::<Vec<u32>>();

      if intersection.is_empty() {
        return 0;
      }
      1
    })
    .sum::<u32>();

  result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = solve_part1(PART_1_EXAMPLE_INPUT);
        assert_eq!(result, "2");
    }

    const PART_2_EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part2_works() {
        let result = solve_part2(PART_2_EXAMPLE_INPUT);
        assert_eq!(result, "4");
    }
}