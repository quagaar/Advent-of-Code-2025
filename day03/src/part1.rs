use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<u32, Error> {
    Ok(input.lines().map(maximum_joltage).sum())
}

fn maximum_joltage(line: &str) -> u32 {
    let bank = line.as_bytes();
    let mut a = 0;
    let mut b = 0;
    for (pos, &ch) in bank.iter().enumerate() {
        if ch > a && pos < bank.len() - 1 {
            a = ch;
            b = 0;
        } else if ch > b {
            b = ch;
            if ch == b'9' {
                break;
            }
        }
    }
    ((a - b'0') * 10 + (b - b'0')) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 357);
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
