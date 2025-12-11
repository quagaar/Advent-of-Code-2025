use std::collections::{HashMap, HashSet};
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
    let mut visited = HashSet::new();
    Ok(dfs(&devices, "svr", false, false, &mut visited))
}

fn dfs<'a>(
    devices: &HashMap<&str, Vec<&'a str>>,
    node: &'a str,
    found_fft: bool,
    found_dac: bool,
    visited: &mut HashSet<&'a str>,
) -> usize {
    if node == "out" {
        if found_fft && found_dac { 1 } else { 0 }
    } else if let Some(next) = devices.get(node) {
        if visited.insert(node) {
            let found_fft = found_fft || node == "fft";
            let found_dac = found_dac || node == "dac";
            let routes = next
                .iter()
                .map(|&next| dfs(devices, next, found_fft, found_dac, visited))
                .sum();
            visited.remove(node);
            routes
        } else {
            0
        }
    } else {
        0
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
