use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    #[error("Invalid direction: {0}")]
    InvalidDirection(char),
    #[error("Empty line encountered")]
    EmptyLine,
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let mut count = 0;
    let mut dial = 50;

    for line in input.lines() {
        match line.chars().next() {
            Some('L') => {
                dial -= line[1..].parse::<i32>()? % 100;
                if dial < 0 {
                    dial += 100;
                }
            }
            Some('R') => {
                dial += line[1..].parse::<i32>()?;
                dial %= 100;
            }
            Some(ch) => {
                return Err(Error::InvalidDirection(ch));
            }
            None => {
                return Err(Error::EmptyLine);
            }
        }

        if dial == 0 {
            count += 1;
        }
    }

    Ok(count)
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
