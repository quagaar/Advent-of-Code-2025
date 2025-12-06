use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),
    #[error("Missing operator")]
    MissingOperator,
}

pub fn solve(input: &str) -> Result<u64, Error> {
    input
        .lines()
        .map(&str::split_ascii_whitespace)
        .fold(Vec::<Vec<&str>>::new(), |mut acc, columns| {
            for (index, value) in columns.enumerate() {
                if let Some(x) = acc.get_mut(index) {
                    x.push(value);
                } else {
                    acc.push(vec![value]);
                }
            }
            acc
        })
        .into_iter()
        .map(|problem| {
            let mut it = problem.into_iter().rev();
            match it.next() {
                Some("+") => it
                    .map(&str::parse::<u64>)
                    .sum::<Result<u64, ParseIntError>>()
                    .map_err(Error::from),
                Some("*") => it
                    .map(&str::parse::<u64>)
                    .product::<Result<u64, ParseIntError>>()
                    .map_err(Error::from),
                Some(op) => Err(Error::InvalidOperator(op.to_owned())),
                None => Err(Error::MissingOperator),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 4277556);
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
