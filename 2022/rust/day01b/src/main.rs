use std::collections::HashMap;

pub fn main() {
  let lines = include_str!("../input.txt")
  .lines();

  let mut elfs = HashMap::new();

  let mut elf_number = 1;
  let mut total_calories_for_this_elf = 0;
  for line in lines {
    if line.is_empty() {
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
  let mut hash_vec: Vec<(&i32, &i32)> = elfs.iter().collect();
  hash_vec.sort_by(|a, b| b.1.cmp(a.1));

  (0..3).for_each(|i| {
    println!("Elf {} has {} calories", hash_vec[i].0, hash_vec[i].1);
  });
  println!("Total: {}", hash_vec[0].1 + hash_vec[1].1 + hash_vec[2].1);
}