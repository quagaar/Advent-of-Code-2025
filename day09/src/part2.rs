use glam::U64Vec2;
use itertools::Itertools;
use std::cmp::Ordering::{Equal, Greater, Less};
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
            // Check if there are no other points inside the rectangle formed by p1 and p2
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
            !coords
                .iter()
                .any(|p| x_range.contains(&p.x) && y_range.contains(&p.y))
        })
        .map(|(p1, p2)| {
            let w = p1.x.abs_diff(p2.x) + 1;
            let h = p1.y.abs_diff(p2.y) + 1;
            w * h
        })
        // .circular_tuple_windows()
        // .map(|(a, b, c)| {
        //     let convex = match (b.x.cmp(&a.x), b.y.cmp(&a.y), b.x.cmp(&c.x), b.y.cmp(&c.y)) {
        //         // Convex corners
        //         (Greater, Equal, Equal, Less) => true,
        //         (Equal, Greater, Greater, Equal) => true,
        //         (Less, Equal, Equal, Greater) => true,
        //         (Equal, Less, Less, Equal) => true,
        //         // Concave corners
        //         (Less, Equal, Equal, Less) => false,
        //         (Equal, Greater, Less, Equal) => false,
        //         (Greater, Equal, Equal, Greater) => false,
        //         (Equal, Less, Greater, Equal) => false,
        //         // Everything else
        //         _ => panic!("Points are not forming a corner"),
        //     };
        //     (*b, convex)
        // })
        // .circular_tuple_windows()
        // .filter(|((p1, _), (_, convex), (p3, _))| {
        //     if *convex {
        //         // Check if there are no other points inside the rectangle formed by p1 and p3
        //         let x_range = if p1.x > p3.x {
        //             p3.x + 1..p1.x
        //         } else {
        //             p1.x + 1..p3.x
        //         };
        //         let y_range = if p1.y > p3.y {
        //             p3.y + 1..p1.y
        //         } else {
        //             p1.y + 1..p3.y
        //         };
        //         !coords
        //             .iter()
        //             .any(|p| x_range.contains(&p.x) && y_range.contains(&p.y))
        //     } else {
        //         false
        //     }
        // })
        // .map(|((p1, _), _, (p3, _))| {
        //     let w = p1.x.abs_diff(p3.x) + 1;
        //     let h = p1.y.abs_diff(p3.y) + 1;
        //     w * h
        // })
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
