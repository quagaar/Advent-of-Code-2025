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
    let (buttons, target_joltages) = parse_line(line)?;
    let mut visited = HashSet::new();
    let mut states = VecDeque::from([(vec![0; target_joltages.len()], 0usize)]);
    while let Some((state, presses)) = states.pop_front() {
        if visited.insert(state.clone()) {
            if state == target_joltages {
                return Ok(presses);
            }
            for button in buttons.iter() {
                if let Some(next_state) = button.iter().try_fold(state.clone(), |mut acc, &i| {
                    if let Some(j) = acc.get_mut(i) {
                        *j += 1;
                        if let Some(t) = target_joltages.get(i)
                            && t >= j
                        {
                            Some(acc)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }) {
                    states.push_back((next_state, presses + 1));
                }
            }
        }
    }
    Err(Error::UnableToReachTarget)
}

fn parse_line(line: &str) -> Result<(Vec<Vec<usize>>, Vec<u16>), Error> {
    let (_lights, remain) = line.split_once("] ").ok_or(Error::MissingDelimiter("] "))?;
    let (buttons, joltages) = remain
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
                .map(|counter| counter.parse().map_err(Error::ParsingError))
                .collect()
        })
        .collect::<Result<Vec<Vec<usize>>, Error>>()?;
    let joltages = joltages
        .strip_suffix("}")
        .ok_or(Error::MissingDelimiter("}"))?
        .split(",")
        .map(|joltage| joltage.parse().map_err(Error::ParsingError))
        .collect::<Result<Vec<u16>, Error>>()?;
    Ok((buttons, joltages))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 33);
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
