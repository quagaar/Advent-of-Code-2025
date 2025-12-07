use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Start marker 'S' not found in input")]
    StartMarkerNotFound,
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let mut lines = input.lines().map(&str::as_bytes);
    let first_line = lines.next().ok_or(Error::EmptyInput)?;
    let start = first_line
        .iter()
        .position(|&c| c == b'S')
        .ok_or(Error::StartMarkerNotFound)?;
    let mut init = vec![0; first_line.len()];
    init[start] = 1usize;
    Ok(lines
        .fold(init, |beams, line| {
            beams
                .into_iter()
                .enumerate()
                .filter(|&(_, worlds)| worlds > 0)
                .fold(
                    vec![0; first_line.len()],
                    |mut new_beams, (beam, worlds)| {
                        if line.get(beam) == Some(&b'^') {
                            new_beams[beam - 1] += worlds;
                            new_beams[beam + 1] += worlds;
                        } else {
                            new_beams[beam] += worlds;
                        }
                        new_beams
                    },
                )
        })
        .into_iter()
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 40);
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
