use pathfinding::prelude::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    #[error("Missing delimiter {0}")]
    MissingDelimiter(&'static str),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let devices = input
        .lines()
        .map(parse_line)
        .collect::<Result<HashMap<_, _>, _>>()?;
    Ok(count_paths(
        &"you",
        |&d| devices.get(d).unwrap(),
        |&d| d == &"out",
    ))
}

fn parse_line(line: &str) -> Result<(&str, Vec<&str>), Error> {
    let (device, outputs) = line.split_once(": ").ok_or(Error::MissingDelimiter(": "))?;
    let outputs = outputs.split_ascii_whitespace().collect();
    Ok((device, outputs))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 5);
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
