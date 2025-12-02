use itertools::Itertools;
use rayon::prelude::*;
use std::ops::{Range, RangeInclusive};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

const ID_VALIDATORS: &[(Range<u64>, u64)] = &[
    (10..100, 11),
    (1000..10_000, 101),
    (100_000..1_000_000, 1001),
    (10_000_000..100_000_000, 10001),
    (1_000_000_000..10_000_000_000, 100001),
    (100_000_000_000..1_000_000_000_000, 1000001),
    (10_000_000_000_000..100_000_000_000_000, 10000001),
    (1_000_000_000_000_000..10_000_000_000_000_000, 100000001),
    (
        100_000_000_000_000_000..1_000_000_000_000_000_000,
        1000000001,
    ),
    (10_000_000_000_000_000_000..u64::MAX, 10000000001),
];

pub fn solve(input: &str) -> Result<u64, Error> {
    input
        .par_split(',')
        .map(|r| Ok(parse_range(r)?.filter(invalid_id).sum::<u64>()))
        .sum()
}

pub fn solve_no_rayon(input: &str) -> Result<u64, Error> {
    input
        .split(',')
        .map(parse_range)
        .flatten_ok()
        .filter_ok(invalid_id)
        .sum()
}

pub fn solve_by_string(input: &str) -> Result<u64, Error> {
    input
        .split(',')
        .map(parse_range)
        .flatten_ok()
        .filter_ok(invalid_id_by_string)
        .sum()
}

fn parse_range(range: &str) -> Result<RangeInclusive<u64>, Error> {
    let range = range.trim();
    let (start, end) = range.split_once('-').unwrap();
    let start: u64 = start.parse()?;
    let end: u64 = end.parse()?;
    Ok(start..=end)
}

fn invalid_id(id: &u64) -> bool {
    ID_VALIDATORS
        .iter()
        .find(|(range, _)| range.contains(id))
        .is_some_and(|(_, divisor)| id.is_multiple_of(*divisor))
}

fn invalid_id_by_string(id: &u64) -> bool {
    let digits = id.to_string();
    let mid = digits.len() / 2;
    let (left, right) = digits.split_at(mid);
    left == right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    const EXAMPLE: &str = include_str!("../example.txt");

    #[template]
    #[rstest]
    #[case(11, true)]
    #[case(1212, true)]
    #[case(123123, true)]
    #[case(12341234, true)]
    #[case(1234512345, true)]
    #[case(123456123456, true)]
    #[case(12345671234567, true)]
    #[case(1234567812345678, true)]
    #[case(123456789123456789, true)]
    #[case(12345678901234567890, true)]
    #[case(12, false)]
    #[case(333, false)]
    #[case(55555, false)]
    #[case(7777777, false)]
    #[case(123454321, false)]
    #[case(1234554321, false)]
    fn id_validator_cases(#[case] id: u64, #[case] expected: bool) {}

    #[apply(id_validator_cases)]
    fn id_validator(#[case] id: u64, #[case] expected: bool) {
        let result = invalid_id(&id);
        assert_eq!(result, expected);
    }

    #[apply(id_validator_cases)]
    fn id_validator_by_string(#[case] id: u64, #[case] expected: bool) {
        let result = invalid_id_by_string(&id);
        assert_eq!(result, expected);
    }

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn example_by_string() {
        let result = solve_by_string(EXAMPLE).unwrap();
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
