use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
struct ParserError;

use crate::utils::Day;

type Coord = i8;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct Droplet(Coord, Coord, Coord);

impl Droplet {
    fn neighbors(&self) -> Vec<Droplet> {
        let (x, y, z) = (self.0, self.1, self.2);

        vec![
            Droplet(x + 1, y, z),
            Droplet(x - 1, y, z),
            Droplet(x, y + 1, z),
            Droplet(x, y - 1, z),
            Droplet(x, y, z + 1),
            Droplet(x, y, z - 1),
        ]
    }
}

impl FromStr for Droplet {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(',');

        if let Some(x_str) = tokens.next() && let Some(y_str) = tokens.next() && let Some(z_str) = tokens.next() {
            if let Ok(x) = x_str.parse::<Coord>() && let Ok(y) = y_str.parse::<Coord>() && let Ok(z) = z_str.parse::<Coord>() {
                Ok(Droplet(x, y, z))
            } else {
                Err(ParserError)
            }
        } else {
            Err(ParserError)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Droplets(Vec<Droplet>);

impl Droplets {
    fn surface_area(&self) -> usize {
        self.iter()
            .map(|droplet| {
                droplet
                    .neighbors()
                    .iter()
                    // Determine how many sides are free
                    .filter(|neighbor| self.0.binary_search(neighbor).is_err())
                    .count()
            })
            .sum()
    }

    fn exterior_surface_area(&self) -> usize {
        // TODO: There can also be holes in the cube that are not closed up
        let mut surface_area = 0;

        let ((x_min, y_min, z_min), (x_max, y_max, z_max)) = self.iter().fold(
            (
                (Coord::MAX, Coord::MAX, Coord::MAX),
                (Coord::MIN, Coord::MIN, Coord::MIN),
            ),
            |((x_min, y_min, z_min), (x_max, y_max, z_max)), droplet| {
                (
                    (
                        x_min.min(droplet.0),
                        y_min.min(droplet.1),
                        z_min.min(droplet.2),
                    ),
                    (
                        x_max.max(droplet.0),
                        y_max.max(droplet.1),
                        z_max.max(droplet.2),
                    ),
                )
            },
        );

        // Check surfaces on x-axis
        for y in y_min..=y_max {
            for z in z_min..=z_max {
                for x in (x_min - 1)..=(x_max + 1) {
                    let droplet = Droplet(x, y, z);

                    if self.binary_search(&droplet).is_ok() {
                        surface_area += 1;
                        break;
                    }
                }

                for x in ((x_min - 1)..=(x_max + 1)).rev() {
                    let droplet = Droplet(x, y, z);

                    if self.binary_search(&droplet).is_ok() {
                        surface_area += 1;
                        break;
                    }
                }
            }
        }

        // Check surfaces on y-axis
        for x in x_min..=x_max {
            for z in z_min..=z_max {
                for y in (y_min - 1)..=(y_max + 1) {
                    let droplet = Droplet(x, y, z);

                    if self.binary_search(&droplet).is_ok() {
                        surface_area += 1;
                        break;
                    }
                }

                for y in ((y_min - 1)..=(y_max + 1)).rev() {
                    let droplet = Droplet(x, y, z);

                    if self.binary_search(&droplet).is_ok() {
                        surface_area += 1;
                        break;
                    }
                }
            }
        }

        // Check surfaces on x-axis
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in (z_min - 1)..=(z_max + 1) {
                    let droplet = Droplet(x, y, z);

                    if self.binary_search(&droplet).is_ok() {
                        surface_area += 1;
                        break;
                    }
                }

                for z in ((z_min - 1)..=(z_max + 1)).rev() {
                    let droplet = Droplet(x, y, z);

                    if self.binary_search(&droplet).is_ok() {
                        surface_area += 1;
                        break;
                    }
                }
            }
        }

        surface_area
    }
}

impl Deref for Droplets {
    type Target = Vec<Droplet>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Droplets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Droplets {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let droplets: Result<Vec<Droplet>, ParserError> = s
            .trim()
            .lines()
            .map(|line| line.parse::<Droplet>())
            .collect();

        let mut droplets = droplets?;
        droplets.sort();

        Ok(Droplets(droplets))
    }
}

pub struct Day18;

impl Day for Day18 {
    fn identifier(&self) -> &'static str {
        "18"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1(&input));
        println!("Part 2: {}", part_2(&input));
    }
}

fn part_1(input: &str) -> usize {
    let droplets: Droplets = input.parse().unwrap();
    droplets.surface_area()
}

fn part_2(input: &str) -> usize {
    let droplets: Droplets = input.parse().unwrap();
    droplets.exterior_surface_area()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn should_calculate_part_1_solution() {
        let actual = part_1(EXAMPLE_INPUT);

        assert_eq!(actual, 64);
    }

    #[test]
    fn should_calculate_part_2_solution() {
        let actual = part_2(EXAMPLE_INPUT);

        assert_eq!(actual, 58);
    }
}
