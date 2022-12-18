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

fn part_2(_input: &str) -> usize {
    0
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

        assert_eq!(actual, 0);
    }
}
