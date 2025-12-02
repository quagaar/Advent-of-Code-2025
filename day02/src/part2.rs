use itertools::Itertools;
use std::ops::{Range, RangeInclusive};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

const ID_VALIDATORS: &[(Range<u64>, &[u64])] = &[
    (10..100, &[11]),
    (100..1000, &[111]),
    (1000..10_000, &[1111, 101]),
    (10_000..100_000, &[11111]),
    (100_000..1_000_000, &[111111, 10101, 1001]),
    (1_000_000..10_000_000, &[1111111]),
    (10_000_000..100_000_000, &[11111111, 1010101, 10001]),
    (100_000_000..1_000_000_000, &[111111111, 1001001]),
    (
        1_000_000_000..10_000_000_000,
        &[1111111111, 101010101, 100001],
    ),
    (10_000_000_000..100_000_000_000, &[11111111111]),
    (
        100_000_000_000..1_000_000_000_000,
        &[111111111111, 10101010101, 1001001001, 100010001, 1000001],
    ),
    (1_000_000_000_000..10_000_000_000_000, &[1111111111111]),
    (
        10_000_000_000_000..100_000_000_000_000,
        &[11111111111111, 1010101010101, 10000001],
    ),
    (
        100_000_000_000_000..1_000_000_000_000_000,
        &[111111111111111, 1001001001001, 10000100001],
    ),
    (
        1_000_000_000_000_000..10_000_000_000_000_000,
        &[1111111111111111, 101010101010101, 1000100010001, 100000001],
    ),
    (
        10_000_000_000_000_000..100_000_000_000_000_000,
        &[11111111111111111],
    ),
    (
        100_000_000_000_000_000..1_000_000_000_000_000_000,
        &[111111111111111111, 10101010101010101, 1000000001],
    ),
    (
        1_000_000_000_000_000_000..10_000_000_000_000_000_000,
        &[1111111111111111111],
    ),
    (
        10_000_000_000_000_000_000..u64::MAX,
        &[
            11111111111111111111,
            1010101010101010101,
            10001000100010001,
            1000010000100001,
            10000000001,
        ],
    ),
];

pub fn solve(input: &str) -> Result<u64, Error> {
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
        .is_some_and(|(_, divisors)| divisors.iter().any(|divisor| id.is_multiple_of(*divisor)))
}

fn invalid_id_by_string(id: &u64) -> bool {
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
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    const EXAMPLE: &str = include_str!("../example.txt");

    #[template]
    #[rstest]
    #[case(22, true)]
    #[case(333, true)]
    #[case(4444, true)]
    #[case(1212, true)]
    #[case(55555, true)]
    #[case(66666, true)]
    #[case(121212, true)]
    #[case(123123, true)]
    #[case(7777777, true)]
    #[case(88888888, true)]
    #[case(12121212, true)]
    #[case(12341234, true)]
    #[case(999999999, true)]
    #[case(123123123, true)]
    #[case(1111111111, true)]
    #[case(1212121212, true)]
    #[case(1234512345, true)]
    #[case(11111111111, true)]
    #[case(111111111111, true)]
    #[case(121212121212, true)]
    #[case(123123123123, true)]
    #[case(123412341234, true)]
    #[case(123456123456, true)]
    #[case(1111111111111, true)]
    #[case(11111111111111, true)]
    #[case(12121212121212, true)]
    #[case(12345671234567, true)]
    #[case(111111111111111, true)]
    #[case(123123123123123, true)]
    #[case(123451234512345, true)]
    #[case(1111111111111111, true)]
    #[case(1212121212121212, true)]
    #[case(1234123412341234, true)]
    #[case(1234567812345678, true)]
    #[case(11111111111111111, true)]
    #[case(111111111111111111, true)]
    #[case(121212121212121212, true)]
    #[case(123456789123456789, true)]
    #[case(1111111111111111111, true)]
    #[case(11111111111111111111, true)]
    #[case(12121212121212121212, true)]
    #[case(12341234123412341234, true)]
    #[case(12345123451234512345, true)]
    #[case(12345678901234567890, true)]
    #[case(12, false)]
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
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn example_by_string() {
        let result = solve_by_string(EXAMPLE).unwrap();
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
