use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    *,
};

use crate::RearrangementInstruction;

// Docs on choosing the correct nom combinators can be found here:
// https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md

pub(crate) fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}

pub(crate) fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(complete::char(' '), parse_crate)(input)?;
    Ok((input, result))
}

pub(crate) fn parse_crates_and_rearrangement_procedure(
    crates_and_rearrangement_procedure: &str,
) -> IResult<&str, (Vec<Vec<&str>>, Vec<RearrangementInstruction>)> {
    let (input, crates_horizontal) =
        separated_list1(newline, parse_crate_line)(crates_and_rearrangement_procedure)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, rearrangements) = separated_list1(newline, parse_rearrangement)(input)?;

    let mut crates_vertical: Vec<Vec<Option<&str>>> = vec![];
    for _ in 0..=crates_horizontal.len() {
        crates_vertical.push(vec![]);
    }
    for vec in crates_horizontal.iter().rev() {
        for (i, c) in vec.iter().enumerate() {
            crates_vertical[i].push(*c)
        }
    }
    let final_crates: Vec<Vec<&str>> = crates_vertical
        .iter()
        .map(|vec| vec.iter().filter_map(|v| *v).collect())
        .collect();

    Ok((input, (final_crates, rearrangements)))
}

pub(crate) fn parse_rearrangement(input: &str) -> IResult<&str, RearrangementInstruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, number) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;
    Ok((
        input,
        RearrangementInstruction {
            amount: number,
            from,
            to,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_crate_works_with_one_crate() {
        let result = parse_crate("[A]");
        assert_eq!(result, Ok(("", Some("A"))));
    }

    #[test]
    fn parse_crate_works_with_empty_crate() {
        let result = parse_crate("   ");
        assert_eq!(result, Ok(("", None)));
    }

    #[test]
    fn parse_line_works_with_full_line() {
        let result = parse_crate_line("[A] [B] [C]");
        assert_eq!(result, Ok(("", vec![Some("A"), Some("B"), Some("C")])));
    }

    #[test]
    fn parse_line_works_with_empty_middle_crate() {
        let result = parse_crate_line("[A]     [C]");
        assert_eq!(result, Ok(("", vec![Some("A"), None, Some("C")])));
    }

    #[test]
    fn parse_rearrangement_works_with_a_single_move() {
        let input = "move 1 from 1 to 2";
        let result = parse_rearrangement(input).unwrap();
        let expected_rearrangement_instruction = RearrangementInstruction {
            amount: 1,
            from: 1,
            to: 2,
        };
        assert_eq!(result.1, expected_rearrangement_instruction);
    }
}
