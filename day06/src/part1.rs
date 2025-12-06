use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),
    #[error("No operators found in input")]
    NoOperators,
}

pub fn solve(input: &str) -> Result<u64, Error> {
    let mut rows = input
        .lines()
        .map(&str::split_ascii_whitespace)
        .collect::<Vec<_>>();
    let ops = rows.pop().ok_or(Error::NoOperators)?;
    ops.map(|op| {
        let numbers = rows
            .iter_mut()
            .filter_map(|row| row.next())
            .map(|s| s.parse::<u64>().map_err(Error::from));
        match op {
            "+" => numbers.sum::<Result<u64, Error>>(),
            "*" => numbers.product::<Result<u64, Error>>(),
            _ => Err(Error::InvalidOperator(op.to_owned())),
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
