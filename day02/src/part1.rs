use itertools::Itertools;
use std::ops::RangeInclusive;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    input
        .split(',')
        .map(parse_range)
        .flatten_ok()
        .filter_ok(invalid_id)
        .sum()
}

fn parse_range(range: &str) -> Result<RangeInclusive<usize>, Error> {
    let range = range.trim();
    let (start, end) = range.split_once('-').unwrap();
    let start: usize = start.parse()?;
    let end: usize = end.parse()?;
    Ok(start..=end)
}

fn invalid_id(id: &usize) -> bool {
    let digits = id.to_string();
    let mid = digits.len() / 2;
    let (left, right) = digits.split_at(mid);
    left == right
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 1227775554);
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
