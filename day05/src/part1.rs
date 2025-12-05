use std::{num::ParseIntError, ops::RangeInclusive};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] ParseIntError),
    #[error("Missing delimiter between ranges and ids")]
    MissingBlankLine,
    #[error("Missing delimiter in range")]
    MissingDelimiter,
}

type Id = u64;
type Ranges = Vec<RangeInclusive<Id>>;

pub fn solve(input: &str) -> Result<usize, Error> {
    let (ranges, ids) = input.split_once("\n\n").ok_or(Error::MissingBlankLine)?;
    let ranges = parse_ranges(ranges)?;

    ids.lines()
        .map(|line| {
            line.parse::<Id>().map_err(Error::from).map(|id| {
                if ranges.iter().any(|range| range.contains(&id)) {
                    1usize
                } else {
                    0usize
                }
            })
        })
        .sum()
}

fn parse_ranges(input: &str) -> Result<Ranges, Error> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').ok_or(Error::MissingDelimiter)?;
            let start: Id = start.parse()?;
            let end: Id = end.parse()?;
            Ok(start..=end)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 3);
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
