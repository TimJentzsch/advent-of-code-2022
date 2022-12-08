use std::str::FromStr;

use crate::utils::Day;

#[derive(Debug)]
struct ParseError;

#[derive(Debug, PartialEq, Eq)]
struct TreeGrid<const R: usize, const C: usize> {
    grid: [[i32; C]; R],
}

impl<const R: usize, const C: usize> TreeGrid<R, C> {
    #[cfg(test)]
    fn new(grid: [[i32; C]; R]) -> Self {
        Self { grid }
    }

    fn create_visibility_map(&self) -> [[bool; C]; R] {
        let mut visibility_map = [[false; C]; R];

        // Check visibility in each direction
        for (row_idx, row) in self.grid.iter().enumerate() {
            // Left to right
            let mut cur_height = -1;

            for (col_idx, &height) in row.iter().enumerate() {
                if cur_height < height {
                    visibility_map[row_idx][col_idx] = true;
                }

                cur_height = cur_height.max(height);
            }

            // Right to left
            let mut cur_height = -1;

            for (col_idx, &height) in row.iter().enumerate().rev() {
                if cur_height < height {
                    visibility_map[row_idx][col_idx] = true;
                }

                cur_height = cur_height.max(height);
            }
        }

        for col_idx in 0..C {
            // Top to bottom
            let mut cur_height = -1;

            for (row_idx, row) in self.grid.iter().enumerate() {
                let height = row[col_idx];

                if cur_height < height {
                    visibility_map[row_idx][col_idx] = true;
                }

                cur_height = cur_height.max(height);
            }

            // Bottom to top
            let mut cur_height = -1;

            for (row_idx, row) in self.grid.iter().enumerate().rev() {
                let height = row[col_idx];

                if cur_height < height {
                    visibility_map[row_idx][col_idx] = true;
                }

                cur_height = cur_height.max(height);
            }
        }

        visibility_map
    }
}

impl<const R: usize, const C: usize> FromStr for TreeGrid<R, C> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [[0i32; C]; R];

        for (row, row_str) in s.trim().lines().enumerate() {
            for (col, height_str) in row_str.chars().enumerate() {
                grid[row][col] = if let Ok(height) = height_str.to_string().parse() {
                    height
                } else {
                    return Err(ParseError);
                }
            }
        }

        Ok(Self { grid })
    }
}

pub struct Day08;

impl Day for Day08 {
    fn identifier(&self) -> &'static str {
        "08"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1::<99, 99>(&input));
        println!("Part 2: {}", part_2(&input));
    }
}

fn part_1<const R: usize, const C: usize>(input: &str) -> usize {
    input
        .parse::<TreeGrid<R, C>>()
        .unwrap()
        .create_visibility_map()
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&visibility| visibility)
        .count()
}

fn part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "30373
25512
65332
33549
35390
";

    #[test]
    fn should_parse_tree_heights() {
        let expected = TreeGrid::new([
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        let actual: TreeGrid<5, 5> = EXAMPLE_INPUT.parse().unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_compute_visibility_map() {
        let grid: TreeGrid<5, 5> = EXAMPLE_INPUT.parse().unwrap();
        let expected = [
            [true, true, true, true, true],
            [true, true, true, false, true],
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, true, true, true, true],
        ];
        let actual = grid.create_visibility_map();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_calculate_part_1_solution() {
        let actual = part_1::<5, 5>(EXAMPLE_INPUT);

        assert_eq!(actual, 21);
    }
}
