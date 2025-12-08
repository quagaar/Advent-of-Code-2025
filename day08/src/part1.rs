use glam::I64Vec3;
use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    #[error("Missing X coordinate")]
    MissingX,
    #[error("Missing Y coordinate")]
    MissingY,
    #[error("Missing Z coordinate")]
    MissingZ,
}

pub fn solve(input: &str, pairs: usize) -> Result<usize, Error> {
    let junction_boxes = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, Error>>()?;
    let mut distances = junction_boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((i, box_a), (j, box_b))| {
            let distance = box_a.distance_squared(*box_b);
            (i, j, distance)
        })
        .collect_vec();
    distances.sort_unstable_by_key(|&(_, _, distance)| distance);
    let circuits = distances.into_iter().take(pairs).fold(
        Vec::new(),
        |mut acc: Vec<Vec<usize>>, (i, j, _)| {
            let a = acc.iter().position(|circuit| circuit.contains(&i));
            let b = acc.iter().position(|circuit| circuit.contains(&j));
            match (a, b) {
                (None, None) => {
                    acc.push(vec![i, j]);
                }
                (Some(a), None) => {
                    acc[a].push(j);
                }
                (None, Some(b)) => {
                    acc[b].push(i);
                }
                (Some(a), Some(b)) if a < b => {
                    let mut other = acc.remove(b);
                    acc[a].append(&mut other);
                }
                (Some(a), Some(b)) if a > b => {
                    let mut other = acc.remove(a);
                    acc[b].append(&mut other);
                }
                _ => {
                    // both already in the same circuit, do nothing
                }
            }
            acc
        },
    );
    Ok(circuits
        .into_iter()
        .map(|v| v.len())
        .sorted()
        .rev()
        .take(3)
        .product())
}

fn parse_line(line: &str) -> Result<I64Vec3, Error> {
    let mut parts = line.split(",");
    let x = parts.next().ok_or(Error::MissingX)?.parse()?;
    let y = parts.next().ok_or(Error::MissingY)?.parse()?;
    let z = parts.next().ok_or(Error::MissingZ)?.parse()?;
    Ok(I64Vec3::new(x, y, z))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE, 10).unwrap();
        assert_eq!(result, 40);
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT, 1000).unwrap();
        assert_eq!(result, expected);
    }
}
