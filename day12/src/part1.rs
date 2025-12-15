use grid::Grid;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing input: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    #[error("Missing delimiter {0} in line")]
    MissingDelimiter(&'static str),
    #[error("Regions not found in input")]
    RegionsNotFound,
    #[error("Shape ID not found in input")]
    ShapeIdNotFound,
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let (shapes, regions) = parse_input(input)?;
    Ok(regions
        .into_par_iter()
        .enumerate()
        .filter(|(index, region)| can_fit(&shapes, region, *index))
        .count())
}

#[allow(unused_variables)]
fn can_fit(shapes: &[Shape], region: &Region, index: usize) -> bool {
    if is_trivial(region) {
        #[cfg(debug_assertions)]
        println!("Region {} can fit by trivial check", index);
        return true;
    }
    if is_impossible(shapes, region) {
        #[cfg(debug_assertions)]
        println!("Region {} can't fit by impossible check", index);
        return false;
    }
    #[cfg(debug_assertions)]
    println!("Region {} requires full check", index);
    let mut counts = region.counts.clone();
    let mut grid = Grid::init(region.length, region.width, b'.');
    can_fit_recursive(shapes, &mut counts, &mut grid)
}

fn is_trivial(region: &Region) -> bool {
    let grid_size = (region.length / 3) * (region.width / 3);
    let total_shapes = region.counts.iter().sum::<usize>();
    total_shapes <= grid_size
}

fn is_impossible(shapes: &[Shape], region: &Region) -> bool {
    let grid_size = region.length * region.width;
    let total_to_fill = region
        .counts
        .iter()
        .zip(shapes)
        .fold(0, |acc, (&count, shape)| {
            acc + count
                * shape.variants[0]
                    .iter()
                    .flatten()
                    .filter(|&&c| c == b'#')
                    .count()
        });
    total_to_fill > grid_size
}

fn can_fit_recursive(shapes: &[Shape], counts: &mut [usize], grid: &mut Grid<u8>) -> bool {
    if counts.iter().all(|&c| c == 0) {
        return true;
    }

    for y in 0..=grid.rows() - 3 {
        for x in 0..=grid.cols() - 3 {
            if valid_location_to_try(grid, x, y) {
                for shape_index in 0..counts.len() {
                    if counts[shape_index] == 0 {
                        continue;
                    }
                    let shape = &shapes[shape_index];
                    for variant in &shape.variants {
                        if can_place_shape(grid, variant, x, y) {
                            place_shape(grid, variant, x, y, b'#');
                            counts[shape_index] -= 1;
                            if can_fit_recursive(shapes, counts, grid) {
                                return true;
                            }
                            counts[shape_index] += 1;
                            place_shape(grid, variant, x, y, b'.');
                        }
                    }
                }
            }
        }
    }
    false
}

fn valid_location_to_try(grid: &Grid<u8>, x: usize, y: usize) -> bool {
    for dy in 0..3 {
        for dx in 0..3 {
            if grid[(y + dy, x + dx)] != b'.' {
                return true;
            }
        }
    }
    if y > 0 && (0..3).all(|dx| grid[(y - 1, x + dx)] == b'.') {
        return false;
    }
    if x > 0 && (0..3).all(|dy| grid[(y + dy, x - 1)] == b'.') {
        return false;
    }
    true
}

fn can_place_shape(grid: &Grid<u8>, shape: &[[u8; 3]; 3], x: usize, y: usize) -> bool {
    for dy in 0..3 {
        for dx in 0..3 {
            if shape[dy][dx] == b'#' && grid[(y + dy, x + dx)] != b'.' {
                return false;
            }
        }
    }
    true
}

fn place_shape(grid: &mut Grid<u8>, shape: &[[u8; 3]; 3], x: usize, y: usize, ch: u8) {
    for dy in 0..3 {
        for dx in 0..3 {
            if shape[dy][dx] == b'#' {
                grid[(y + dy, x + dx)] = ch;
            }
        }
    }
}

struct Shape {
    _id: usize,
    variants: Vec<[[u8; 3]; 3]>,
}

struct Region {
    width: usize,
    length: usize,
    counts: Vec<usize>,
}

fn parse_input(input: &str) -> Result<(Vec<Shape>, Vec<Region>), Error> {
    let mut sections = input.split("\n\n").collect::<Vec<_>>();
    let regions = sections
        .pop()
        .ok_or(Error::RegionsNotFound)?
        .lines()
        .map(parse_region)
        .collect::<Result<Vec<_>, _>>()?;
    let shapes = sections
        .into_iter()
        .map(parse_shape)
        .collect::<Result<Vec<_>, _>>()?;
    Ok((shapes, regions))
}

fn parse_shape(section: &str) -> Result<Shape, Error> {
    let mut lines = section.lines();
    let _id = lines
        .next()
        .ok_or(Error::ShapeIdNotFound)?
        .strip_suffix(":")
        .ok_or(Error::MissingDelimiter(":"))?
        .parse()?;
    let mut shape = [[0u8; 3]; 3];
    for (y, line) in lines.enumerate().take(3) {
        for (x, ch) in line.as_bytes().iter().enumerate().take(3) {
            shape[y][x] = *ch;
        }
    }
    Ok(Shape {
        _id,
        variants: get_variants(shape),
    })
}

fn get_variants(shape: [[u8; 3]; 3]) -> Vec<[[u8; 3]; 3]> {
    let r1 = rotate_90(&shape);
    let r2 = rotate_90(&r1);
    let r3 = rotate_90(&r2);
    let f1 = flip_horizontal(&shape);
    let f2 = rotate_90(&f1);
    let f3 = rotate_90(&f2);
    let f4 = rotate_90(&f3);
    [shape, r1, r2, r3, f1, f2, f3, f4]
        .into_iter()
        .fold(Vec::new(), |mut acc, v| {
            if !acc.contains(&v) {
                acc.push(v);
            }
            acc
        })
}

fn rotate_90(shape: &[[u8; 3]; 3]) -> [[u8; 3]; 3] {
    let mut rotated = [[0u8; 3]; 3];
    for y in 0..3 {
        for x in 0..3 {
            rotated[x][2 - y] = shape[y][x];
        }
    }
    rotated
}

fn flip_horizontal(shape: &[[u8; 3]; 3]) -> [[u8; 3]; 3] {
    let mut flipped = [[0u8; 3]; 3];
    for y in 0..3 {
        for x in 0..3 {
            flipped[y][2 - x] = shape[y][x];
        }
    }
    flipped
}

fn parse_region(line: &str) -> Result<Region, Error> {
    let (size, counts) = line.split_once(": ").ok_or(Error::MissingDelimiter(": "))?;
    let (width, length) = size.split_once('x').ok_or(Error::MissingDelimiter("x"))?;
    let width = width.parse::<usize>().map_err(Error::from)?;
    let length = length.parse::<usize>().map_err(Error::from)?;
    let counts = counts
        .split_ascii_whitespace()
        .map(&str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Region {
        width,
        length,
        counts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 2);
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
