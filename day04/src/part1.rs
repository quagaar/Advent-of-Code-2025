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
            if cell == b'@' && count_rolls_in_area(&grid, x, y) < 4 + 1 {
                accessible += 1;
            }
        }
    }

    Ok(accessible)
}

/// Count the number of rolls ('@') in the 3x3 area centered at (x, y)
/// including the cell at (x, y) itself.
fn count_rolls_in_area(grid: &[&[u8]], x: usize, y: usize) -> usize {
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
