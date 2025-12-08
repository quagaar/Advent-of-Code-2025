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

pub fn solve(input: &str) -> Result<i64, Error> {
    let junction_boxes = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, Error>>()?;
    let mut distances = junction_boxes
        .iter()
        .enumerate()
        .combinations(2)
        .map(|pair| {
            let (i, box_a) = pair[0];
            let (j, box_b) = pair[1];
            let distance = box_a.distance_squared(*box_b);
            (i, j, distance)
        })
        .collect_vec();
    distances.sort_by_key(|&(_, _, distance)| distance);
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for (i, j, _) in distances.into_iter() {
        let a = circuits.iter().position(|circuit| circuit.contains(&i));
        let b = circuits.iter().position(|circuit| circuit.contains(&j));
        match (a, b) {
            (None, None) => {
                circuits.push(vec![i, j]);
            }
            (Some(a), None) => {
                circuits[a].push(j);
            }
            (None, Some(b)) => {
                circuits[b].push(i);
            }
            (Some(a), Some(b)) if a < b => {
                let mut other = circuits.remove(b);
                circuits[a].append(&mut other);
            }
            (Some(a), Some(b)) if a > b => {
                let mut other = circuits.remove(a);
                circuits[b].append(&mut other);
            }
            _ => {
                // both already in the same circuit, do nothing
            }
        }
        if circuits.len() == 1 && circuits[0].len() == junction_boxes.len() {
            return Ok(junction_boxes[i].x * junction_boxes[j].x);
        }
    }
    unreachable!("Should not reach here");
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
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 25272);
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
