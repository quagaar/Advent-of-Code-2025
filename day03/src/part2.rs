use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<u64, Error> {
    Ok(input.lines().map(maximum_joltage).sum())
}

fn maximum_joltage(line: &str) -> u64 {
    let bank = line.as_bytes();
    let mut digits = [0; 12];
    let mut top = 0;
    for (i, digit) in digits.iter_mut().enumerate() {
        for (pos, &ch) in bank.iter().enumerate().take(bank.len() - 11 + i).skip(top) {
            if ch > *digit {
                *digit = ch;
                top = pos + 1;
                if ch == b'9' {
                    break;
                }
            }
        }
    }
    digits
        .into_iter()
        .fold(0u64, |acc, d| acc * 10 + (d - b'0') as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 3121910778619);
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
