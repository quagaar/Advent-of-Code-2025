use glam::U64Vec2;
use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    #[error("Missing comma in line")]
    MissingComma,
    #[error("No valid result found")]
    NotFound,
}

pub fn solve(input: &str) -> Result<u64, Error> {
    let coords: Vec<_> = input.lines().map(parse_line).try_collect()?;
    coords
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let w = a.x.abs_diff(b.x) + 1;
            let h = a.y.abs_diff(b.y) + 1;
            w * h
        })
        .max()
        .ok_or(Error::NotFound)
}

fn parse_line(line: &str) -> Result<U64Vec2, Error> {
    let (x, y) = line.split_once(',').ok_or(Error::MissingComma)?;
    let x = x.parse()?;
    let y = y.parse()?;
    Ok(U64Vec2::new(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 50);
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
