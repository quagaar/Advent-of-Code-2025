use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().to_owned())
        .collect::<Vec<_>>();
    let width = grid[0].len();

    Ok(std::iter::repeat_with(|| remove_rolls(&mut grid, width))
        .take_while(|&removed| removed > 0)
        .sum())
}

/// Remove rolls ('@') that have less than 4 neighboring rolls in their 3x3 area.
/// Returns the number of rolls removed in this pass.
fn remove_rolls(grid: &mut [Vec<u8>], width: usize) -> usize {
    let mut removed = 0;

    for y in 0..grid.len() {
        for x in 0..width {
            if grid[y][x] == b'@' && count_adjacent(grid, x, y) < 4 {
                grid[y][x] = b'x';
                removed += 1;
            }
        }
    }

    removed
}

const ADJACENT: &[(isize, &[isize])] = &[(-1, &[-1, 0, 1]), (0, &[-1, 1]), (1, &[-1, 0, 1])];

/// Count the number of rolls ('@') adjacent to (x, y)
/// (horizontally, vertically, and diagonally).
fn count_adjacent(grid: &[Vec<u8>], x: usize, y: usize) -> usize {
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
        assert_eq!(result, 43);
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
