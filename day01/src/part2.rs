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
                if dial == 0 {
                    count -= 1;
                }
                dial -= line[1..].parse::<i32>()?;
                while dial < 0 {
                    dial += 100;
                    count += 1;
                }
                if dial == 0 {
                    count += 1;
                }
            }
            Some('R') => {
                dial += line[1..].parse::<i32>()?;
                while dial >= 100 {
                    dial -= 100;
                    count += 1;
                }
            }
            Some(ch) => {
                return Err(Error::InvalidDirection(ch));
            }
            None => {
                return Err(Error::EmptyLine);
            }
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
        assert_eq!(result, 6);
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
