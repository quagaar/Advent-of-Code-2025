use rayon::prelude::*;
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

pub fn solve(input: &str) -> Result<u16, Error> {
    input.par_lines().map(process_line).sum()
}

fn process_line(line: &str) -> Result<u16, Error> {
    let (buttons, target_joltages) = parse_line(line)?;
    dfs(
        &vec![0; target_joltages.len()],
        &target_joltages,
        &buttons,
        0,
        u16::MAX,
    )
    .ok_or(Error::UnableToReachTarget)
}

fn dfs(
    joltages: &[u16],
    target: &[u16],
    buttons: &[Vec<usize>],
    count: u16,
    mut best: u16,
) -> Option<u16> {
    if let Some(button) = buttons.first() {
        for n in 0.. {
            let mut next_joltages = joltages.to_vec();
            for &i in button {
                next_joltages[i] += n;
            }

            if count + n >= best {
                break;
            } else if next_joltages.iter().zip(target).all(|(a, b)| a == b) {
                best = count + n;
                break;
            } else if next_joltages.iter().zip(target).any(|(a, b)| a > b) {
                break;
            } else if let Some(steps) = dfs(&next_joltages, target, &buttons[1..], count + n, best)
                && steps < best
            {
                best = steps;
            }
        }
        if best == u16::MAX { None } else { Some(best) }
    } else {
        None
    }
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
