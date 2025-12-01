use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    #[error("Invalid movement: {0}")]
    InvalidMovement(String),
}

enum Movement {
    Left(i32),
    Right(i32),
}

pub fn solve(input: &str) -> Result<i32, Error> {
    let mut count = 0;
    let mut dial = 50;

    for line in input.lines() {
        match parse_line(line)? {
            Movement::Left(ticks) => {
                count += ticks / 100;
                let n = dial - (ticks % 100);
                if dial != 0 && n <= 0 {
                    count += 1;
                }
                dial = if n < 0 { n + 100 } else { n };
            }
            Movement::Right(ticks) => {
                let n = dial + ticks;
                count += n / 100;
                dial = n % 100;
            }
        }
    }

    Ok(count)
}

fn parse_line(line: &str) -> Result<Movement, Error> {
    match line.split_at_checked(1) {
        Some(("L", ticks)) => Ok(Movement::Left(ticks.parse()?)),
        Some(("R", ticks)) => Ok(Movement::Right(ticks.parse()?)),
        _ => Err(Error::InvalidMovement(line.to_owned())),
    }
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
