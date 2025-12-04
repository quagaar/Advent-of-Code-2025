use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let grid = input.lines().map(&str::as_bytes).collect::<Vec<_>>();
    let mut accessible = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == b'@' && count_adjacent(&grid, x, y) < 4 {
                accessible += 1;
            }
        }
    }

    Ok(accessible)
}

const ADJACENT: &[(isize, &[isize])] = &[(-1, &[-1, 0, 1]), (0, &[-1, 1]), (1, &[-1, 0, 1])];

/// Count the number of rolls ('@') adjacent to (x, y)
/// (horizontally, vertically, and diagonally).
fn count_adjacent(grid: &[&[u8]], x: usize, y: usize) -> usize {
    ADJACENT
        .iter()
        .filter_map(|(dy, dxs)| {
            y.checked_add_signed(*dy)
                .and_then(|y| grid.get(y).map(|row| (row, dxs)))
        })
        .flat_map(|(row, dxs)| {
            dxs.iter()
                .filter_map(|&dx| x.checked_add_signed(dx).and_then(|x| row.get(x)))
        })
        .filter(|&&cell| cell == b'@')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 13);
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
