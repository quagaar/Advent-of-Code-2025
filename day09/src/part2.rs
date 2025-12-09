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
        .iter()
        .tuple_combinations()
        .filter(|(p1, p2)| {
            // Check if there are no other lines crossing rectangle formed by p1 and p2
            let x_range = if p1.x > p2.x {
                p2.x + 1..p1.x
            } else {
                p1.x + 1..p2.x
            };
            let y_range = if p1.y > p2.y {
                p2.y + 1..p1.y
            } else {
                p1.y + 1..p2.y
            };
            !coords.iter().circular_tuple_windows().any(|(a, b)| {
                // Check if line segment (a, b) crosses the rectangle
                if a.x == b.x {
                    // vertical line
                    x_range.contains(&a.x)
                        && a.y.min(b.y) < y_range.end
                        && a.y.max(b.y) >= y_range.start
                } else {
                    // horizontal line
                    y_range.contains(&a.y)
                        && a.x.min(b.x) < x_range.end
                        && a.x.max(b.x) >= x_range.start
                }
            })
        })
        .map(|(p1, p2)| {
            let w = p1.x.abs_diff(p2.x) + 1;
            let h = p1.y.abs_diff(p2.y) + 1;
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
        assert_eq!(result, 24);
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
