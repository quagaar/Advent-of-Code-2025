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
    let mut init_beams = vec![false; first_line.len()];
    init_beams[start] = true;
    let (splits, _) = lines.fold((0, init_beams), |(mut splits, beams), line| {
        let new_beams = beams.into_iter().enumerate().filter(|&(_, c)| c).fold(
            vec![false; first_line.len()],
            |mut new_beams, (beam, _)| {
                if line.get(beam) == Some(&b'^') {
                    new_beams[beam - 1] = true;
                    new_beams[beam + 1] = true;
                    splits += 1;
                } else {
                    new_beams[beam] = true;
                }
                new_beams
            },
        );
        (splits, new_beams)
    });
    Ok(splits)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 21);
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
