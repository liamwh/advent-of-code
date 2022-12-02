use std::collections::HashMap;

pub fn main() {
  let lines = include_str!("../input.txt")
  .lines();

  let mut elfs = HashMap::new();


  let mut max_calories_seen = 0;
  let mut elf_with_max_calories = 0;

  let mut elf_number = 1;
  let mut total_calories_for_this_elf = 0;
  for line in lines {
    if line.is_empty() {
      if total_calories_for_this_elf > max_calories_seen {
        max_calories_seen = total_calories_for_this_elf;
        elf_with_max_calories = elf_number;
      }
      elfs.insert(elf_number, total_calories_for_this_elf);
      total_calories_for_this_elf = 0;
      elf_number += 1;
    }
    else {
      let mut split = line.split(' ');
      let calories = split.next().unwrap().parse::<i32>().unwrap();
      total_calories_for_this_elf += calories;
    }
  }
  println!("Elf {} has the most calories: {}", elf_with_max_calories, max_calories_seen);
}