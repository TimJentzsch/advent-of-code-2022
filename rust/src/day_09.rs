#![allow(clippy::bool_to_int_with_if)]

use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::utils::Day;

#[derive(Debug)]
struct ParseError;

#[derive(Debug, PartialEq, Eq)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Motion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let motion = match s {
            "U" => Motion::Up,
            "D" => Motion::Down,
            "L" => Motion::Left,
            "R" => Motion::Right,
            _ => return Err(ParseError),
        };

        Ok(motion)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    motion: Motion,
    count: usize,
}

impl Instruction {
    #[cfg(test)]
    fn new(motion: Motion, count: usize) -> Self {
        Self { motion, count }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (motion_str, count_str) = if let Some(parts) = s.split_once(' ') {
            parts
        } else {
            return Err(ParseError);
        };

        let motion = motion_str.parse()?;
        let count = count_str.parse().map_err(|_| ParseError)?;

        Ok(Self { motion, count })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    #[cfg(test)]
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn apply_motion(&mut self, motion: &Motion) -> &mut Self {
        match motion {
            Motion::Up => self.y += 1,
            Motion::Down => self.y -= 1,
            Motion::Left => self.x -= 1,
            Motion::Right => self.x += 1,
        };

        self
    }

    fn is_adjacent_to(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn follow(&mut self, other: &Position) -> &mut Self {
        if self.is_adjacent_to(other) {
            return self;
        }

        let x_delta = match self.x.cmp(&other.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let y_delta = match self.y.cmp(&other.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        self.x += x_delta;
        self.y += y_delta;

        self
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rope<const L: usize> {
    knots: [Position; L],
    visited_by_tail: HashSet<Position>,
}

impl<const L: usize> Rope<L> {
    #[cfg(test)]
    fn head(&self) -> Position {
        self.knots[0]
    }

    #[cfg(test)]
    fn tail(&self) -> Position {
        self.knots[L - 1]
    }

    fn apply_motion(&mut self, motion: &Motion) -> &mut Self {
        self.knots[0].apply_motion(motion);

        for idx in 1..L {
            let to_follow = self.knots[idx - 1];
            self.knots[idx].follow(&to_follow);
        }

        self.visited_by_tail.insert(self.knots[L - 1]);
        self
    }

    fn apply_instruction(&mut self, instruction: &Instruction) -> &mut Self {
        for _ in 0..instruction.count {
            self.apply_motion(&instruction.motion);
        }

        self
    }

    fn visited_by_tail_count(&self) -> usize {
        self.visited_by_tail.len()
    }
}

impl<const L: usize> Default for Rope<L> {
    fn default() -> Self {
        Self {
            knots: [Position::default(); L],
            visited_by_tail: HashSet::default(),
        }
    }
}

pub struct Day09;

impl Day for Day09 {
    fn identifier(&self) -> &'static str {
        "09"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1(&input));
        println!("Part 2: {}", part_2(&input));
    }
}

fn part_1(input: &str) -> usize {
    let mut rope = Rope::<2>::default();

    for line in input.trim().lines() {
        let instruction: Instruction = line.parse().unwrap();
        rope.apply_instruction(&instruction);
    }

    rope.visited_by_tail_count()
}

fn part_2(input: &str) -> usize {
    let mut rope = Rope::<10>::default();

    for line in input.trim().lines() {
        let instruction: Instruction = line.parse().unwrap();
        rope.apply_instruction(&instruction);
    }

    rope.visited_by_tail_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const EXAMPLE_INPUT_2: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn should_calculate_part_1_solution() {
        let actual = part_1(EXAMPLE_INPUT_1);

        assert_eq!(actual, 13);
    }

    #[test]
    fn should_follow_across_sides() {
        let head = Position::new(3, 1);
        let mut tail = Position::new(1, 1);
        tail.follow(&head);

        assert_eq!(tail, Position::new(2, 1));

        let head = Position::new(1, 1);
        let mut tail = Position::new(1, 3);
        tail.follow(&head);

        assert_eq!(tail, Position::new(1, 2));
    }

    #[test]
    fn should_follow_across_diagonals() {
        let head = Position::new(4, 2);
        let mut tail = Position::new(3, 0);
        tail.follow(&head);

        assert_eq!(tail, Position::new(4, 1));

        let head = Position::new(2, 3);
        let mut tail = Position::new(1, 1);
        tail.follow(&head);

        assert_eq!(tail, Position::new(2, 2));
    }

    #[test]
    fn should_apply_instructions() {
        let mut rope = Rope::<2>::default();

        // R 4
        rope.apply_instruction(&Instruction::new(Motion::Right, 4));
        assert_eq!(rope.head(), Position::new(4, 0));
        assert_eq!(rope.tail(), Position::new(3, 0));
        assert_eq!(rope.visited_by_tail_count(), 4);

        // U 4
        rope.apply_instruction(&Instruction::new(Motion::Up, 4));
        assert_eq!(rope.head(), Position::new(4, 4));
        assert_eq!(rope.tail(), Position::new(4, 3));
        assert_eq!(rope.visited_by_tail_count(), 7);

        // L 3
        rope.apply_instruction(&Instruction::new(Motion::Left, 3));
        assert_eq!(rope.head(), Position::new(1, 4));
        assert_eq!(rope.tail(), Position::new(2, 4));
        assert_eq!(rope.visited_by_tail_count(), 9);

        // D 1
        rope.apply_instruction(&Instruction::new(Motion::Down, 1));
        assert_eq!(rope.head(), Position::new(1, 3));
        assert_eq!(rope.tail(), Position::new(2, 4));
        assert_eq!(rope.visited_by_tail_count(), 9);

        // R 4
        rope.apply_instruction(&Instruction::new(Motion::Right, 4));
        assert_eq!(rope.head(), Position::new(5, 3));
        assert_eq!(rope.tail(), Position::new(4, 3));
        assert_eq!(rope.visited_by_tail_count(), 10);

        // D 1
        rope.apply_instruction(&Instruction::new(Motion::Down, 1));
        assert_eq!(rope.head(), Position::new(5, 2));
        assert_eq!(rope.tail(), Position::new(4, 3));
        assert_eq!(rope.visited_by_tail_count(), 10);

        // L 5
        rope.apply_instruction(&Instruction::new(Motion::Left, 5));
        assert_eq!(rope.head(), Position::new(0, 2));
        assert_eq!(rope.tail(), Position::new(1, 2));
        assert_eq!(rope.visited_by_tail_count(), 13);

        // R 2
        rope.apply_instruction(&Instruction::new(Motion::Right, 2));
        assert_eq!(rope.head(), Position::new(2, 2));
        assert_eq!(rope.tail(), Position::new(1, 2));
        assert_eq!(rope.visited_by_tail_count(), 13);
    }

    #[test]
    fn should_calculate_part_2_solution_example_1() {
        let actual = part_2(EXAMPLE_INPUT_1);

        assert_eq!(actual, 1);
    }

    #[test]
    fn should_calculate_part_2_solution_example_2() {
        let actual = part_2(EXAMPLE_INPUT_2);

        assert_eq!(actual, 36);
    }
}
