mod parsers;

#[derive(PartialEq, Debug)]
pub struct RearrangementInstruction {
    pub amount: u32,
    pub from: u32,
    pub to: u32,
}

// I considered making a struct to represent a stack, however I decided to only implement this if the 2nd part of the challenge
// provided the stacks in an unordered fashion, and therefore requiring a hashmap with stack IDs as keys.
// As both challenges provided the stacks in the correct order each time however, a vector was sufficient, so I didn't create the struct.

pub fn solve_part1(crates_and_rearrangement_procedure: &str) -> String {
    let (_, (mut crate_stacks, rearrangement_instructions)) =
        parsers::parse_crates_and_rearrangement_procedure(crates_and_rearrangement_procedure)
            .unwrap();
    for instruction in rearrangement_instructions.iter() {
        let from_index = instruction.from as usize - 1;
        let to_index = instruction.to as usize - 1;
        let len = crate_stacks[from_index].len();
        let drained = crate_stacks[from_index]
            .drain((len - instruction.amount as usize)..)
            .rev()
            .collect::<Vec<&str>>();
        for c in drained.iter() {
            crate_stacks[to_index].push(c);
        }
    }

    let crates_on_top_of_stacks: String = crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    crates_on_top_of_stacks
}

pub fn solve_part2(crates_and_rearrangement_procedure: &str) -> String {
    let (_, (mut crate_stacks, rearrangement_instructions)) =
        parsers::parse_crates_and_rearrangement_procedure(crates_and_rearrangement_procedure)
            .unwrap();
    for instruction in rearrangement_instructions.iter() {
        let from_index = instruction.from as usize - 1;
        let to_index = instruction.to as usize - 1;
        let len = crate_stacks[from_index].len();
        let drained = crate_stacks[from_index]
            .drain((len - instruction.amount as usize)..)
            .collect::<Vec<&str>>();
        for c in drained.iter() {
            crate_stacks[to_index].push(c);
        }
    }

    let crates_on_top_of_stacks: String = crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    crates_on_top_of_stacks
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    // For these tests I get the content from the example-input file instead of a const,
    // so my IDE would stop automatically trimming the whitespace from the end of the line
    #[test]
    fn part1_works() {
        let file = fs::read_to_string("./example-input.txt").unwrap();
        let result = solve_part1(&file);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let file = fs::read_to_string("./example-input.txt").unwrap();
        let result = solve_part2(&file);
        assert_eq!(result, "MCD");
    }
}
