use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    #[error("Invalid operator: {0}")]
    InvalidOperator(u8),
    #[error("No operators found in input")]
    NoOperators,
}

pub fn solve(input: &str) -> Result<u64, Error> {
    let mut lines = input.lines().map(&str::as_bytes).collect::<Vec<_>>();
    lines
        .pop()
        .ok_or(Error::NoOperators)?
        .iter()
        .enumerate()
        .filter(|(_, ch)| !ch.is_ascii_whitespace())
        .map(|(index, op)| {
            let numbers = (index..).map_while(|index| {
                lines
                    .iter()
                    .filter_map(|line| line.get(index))
                    .fold(None, |acc, &ch| {
                        if ch.is_ascii_digit() {
                            Some(acc.unwrap_or(0) * 10 + (ch - b'0') as u64)
                        } else {
                            acc
                        }
                    })
            });
            match op {
                b'+' => Ok(numbers.sum::<u64>()),
                b'*' => Ok(numbers.product()),
                _ => Err(Error::InvalidOperator(*op)),
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
        assert_eq!(result, 3263827);
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
