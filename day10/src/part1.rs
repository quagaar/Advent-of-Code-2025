use std::collections::{HashSet, VecDeque};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    #[error("Missing delimiter: {0}")]
    MissingDelimiter(&'static str),
    #[error("Unable to reach target configuration")]
    UnableToReachTarget,
}

pub fn solve(input: &str) -> Result<usize, Error> {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> Result<usize, Error> {
    let (lights, buttons) = parse_line(line)?;
    let mut visited = HashSet::new();
    let mut states = VecDeque::from([(0u16, 0usize)]);
    while let Some((state, presses)) = states.pop_front() {
        if visited.insert(state) {
            if state == lights {
                return Ok(presses);
            }
            for &button in buttons.iter() {
                let next_state = state ^ button;
                states.push_back((next_state, presses + 1));
            }
        }
    }
    Err(Error::UnableToReachTarget)
}

fn parse_line(line: &str) -> Result<(u16, Vec<u16>), Error> {
    let (lights, remain) = line.split_once("] ").ok_or(Error::MissingDelimiter("] "))?;
    let lights = lights
        .strip_prefix("[")
        .ok_or(Error::MissingDelimiter("["))?
        .as_bytes()
        .iter()
        .rev()
        .fold(0, |acc, &b| (acc << 1) | if b == b'#' { 1 } else { 0 });
    let (buttons, _joltages) = remain
        .split_once(" {")
        .ok_or(Error::MissingDelimiter(" {"))?;
    let buttons = buttons
        .split_ascii_whitespace()
        .map(|button| {
            button
                .strip_prefix("(")
                .ok_or(Error::MissingDelimiter("("))?
                .strip_suffix(")")
                .ok_or(Error::MissingDelimiter(")"))?
                .split(",")
                .try_fold(0, |acc, num_str| {
                    let num: usize = num_str.parse()?;
                    Ok(acc | (1 << num))
                })
        })
        .collect::<Result<Vec<u16>, Error>>()?;
    Ok((lights, buttons))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 7);
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
