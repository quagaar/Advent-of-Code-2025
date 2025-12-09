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
    let (mut horizontal_lines, mut vertical_lines) = coords.iter().circular_tuple_windows().fold(
        (Vec::new(), Vec::new()),
        |(mut h_lines, mut v_lines), (p1, p2)| {
            if p1.x == p2.x {
                v_lines.push((p1.x, p1.y.min(p2.y), p1.y.max(p2.y)));
            } else if p1.y == p2.y {
                h_lines.push((p1.y, p1.x.min(p2.x), p1.x.max(p2.x)));
            }
            (h_lines, v_lines)
        },
    );
    horizontal_lines.sort_unstable();
    vertical_lines.sort_unstable();
    coords
        .into_iter()
        .tuple_combinations()
        .filter(|(p1, p2)| {
            // Check if there are no other lines crossing rectangle formed by p1 and p2
            let x_min = p1.x.min(p2.x);
            let x_max = p1.x.max(p2.x);
            let y_min = p1.y.min(p2.y);
            let y_max = p1.y.max(p2.y);
            !horizontal_lines
                .iter()
                .skip_while(|(y, _, _)| *y <= y_min)
                .take_while(|(y, _, _)| *y < y_max)
                .any(|(_, x1, x2)| *x1 < x_max && *x2 > x_min)
                && !vertical_lines
                    .iter()
                    .skip_while(|(x, _, _)| *x <= x_min)
                    .take_while(|(x, _, _)| *x < x_max)
                    .any(|(_, y1, y2)| *y1 < y_max && *y2 > y_min)
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
