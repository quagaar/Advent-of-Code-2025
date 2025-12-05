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
    let (ranges, _) = input.split_once("\n\n").ok_or(Error::MissingBlankLine)?;
    let ranges = merge_ranges(parse_ranges(ranges)?);
    Ok(ranges.into_iter().map(|range| range.count()).sum())
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

fn merge_ranges(ranges: Ranges) -> Ranges {
    ranges
        .into_iter()
        .fold(Ranges::new(), |mut merged, mut range| {
            merged.retain(|r| {
                if range.start().saturating_sub(*r.end()) <= 1
                    && r.start().saturating_sub(*range.end()) <= 1
                {
                    let start = *range.start().min(r.start());
                    let end = *range.end().max(r.end());
                    range = start..=end;
                    false
                } else {
                    true
                }
            });
            merged.push(range);
            merged
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 14);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
