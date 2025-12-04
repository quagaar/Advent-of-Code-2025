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
            if grid[y][x] == b'@' && count_rolls_in_area(grid, x, y) < 4 + 1 {
                grid[y][x] = b'x';
                removed += 1;
            }
        }
    }

    removed
}

/// Count the number of rolls ('@') in the 3x3 area centered at (x, y)
/// including the cell at (x, y) itself.
fn count_rolls_in_area(grid: &[Vec<u8>], x: usize, y: usize) -> usize {
    grid.iter()
        .take(y + 2)
        .skip(y.saturating_sub(1))
        .flat_map(|row| row.iter().take(x + 2).skip(x.saturating_sub(1)))
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
