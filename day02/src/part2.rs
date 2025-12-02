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
    let len = digits.len();
    for n in 1..=(len / 2) {
        if len.is_multiple_of(n) && digits_repeat_every_n_characters(&digits, n) {
            return true;
        }
    }
    false
}

fn digits_repeat_every_n_characters(digits: &str, n: usize) -> bool {
    let (a, b) = digits.split_at(n);
    a == b || (b.starts_with(a) && digits_repeat_every_n_characters(b, n))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 4174379265);
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
