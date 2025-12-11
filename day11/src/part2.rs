use pathfinding::prelude::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing delimiter {0}")]
    MissingDelimiter(&'static str),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let devices = input
        .lines()
        .map(parse_line)
        .collect::<Result<HashMap<_, _>, _>>()?;
    let empty = Vec::new();
    let successors = |&d| devices.get(d).unwrap_or(&empty);
    let fft_paths = count_paths(
        &"svr",
        |&d| successors(d).iter().filter(|&&d| d != "dac"),
        |&d| d == &"fft",
    );

    if fft_paths > 0 {
        let dac_paths = count_paths(&"fft", |&d| successors(d), |&d| d == &"dac");
        let out_paths = count_paths(&"dac", |&d| successors(d), |&d| d == &"out");
        Ok(fft_paths * dac_paths * out_paths)
    } else {
        let dac_paths = count_paths(&"svr", |&d| successors(d), |&d| d == &"dac");
        let fft_paths = count_paths(&"dac", |&d| successors(d), |&d| d == &"fft");
        let out_paths = count_paths(&"fft", |&d| successors(d), |&d| d == &"out");
        Ok(dac_paths * fft_paths * out_paths)
    }
}

fn parse_line(line: &str) -> Result<(&str, Vec<&str>), Error> {
    let (device, outputs) = line.split_once(": ").ok_or(Error::MissingDelimiter(": "))?;
    let outputs = outputs.split_ascii_whitespace().collect();
    Ok((device, outputs))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 2);
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
