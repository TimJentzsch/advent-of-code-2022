use std::str::FromStr;

use crate::utils::Day;

#[derive(Debug)]
struct ParseError;

#[derive(Debug, PartialEq, Eq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseError;

    /// Parses input in the form `move 3 from 9 to 6`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        if let Some("move") = tokens.next() {
        } else {
            return Err(ParseError);
        };

        let count = if let Some(count_str) = tokens.next() && let Ok(count) = count_str.parse::<usize>() {
            count
        } else {
            return Err(ParseError);
        };

        if let Some("from") = tokens.next() {
        } else {
            return Err(ParseError);
        };

        let from = if let Some(from_str) = tokens.next() && let Ok(from) = from_str.parse::<usize>() {
            from
        } else {
            return Err(ParseError);
        };

        if let Some("to") = tokens.next() {
        } else {
            return Err(ParseError);
        };

        let to = if let Some(to_str) = tokens.next() && let Ok(to) = to_str.parse::<usize>() {
            to
        } else {
            return Err(ParseError);
        };

        Ok(Move { count, from, to })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Crate(char);

impl From<char> for Crate {
    fn from(value: char) -> Self {
        Crate(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CrateStack(Vec<Crate>);

impl CrateStack {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, value: Crate) {
        self.0.push(value)
    }

    fn pop(&mut self) -> Option<Crate> {
        self.0.pop()
    }

    fn top(&self) -> Option<&Crate> {
        self.0.last()
    }
}

impl FromIterator<char> for CrateStack {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        CrateStack(iter.into_iter().map(Crate::from).collect())
    }
}

impl FromIterator<Crate> for CrateStack {
    fn from_iter<T: IntoIterator<Item = Crate>>(iter: T) -> Self {
        CrateStack(iter.into_iter().collect())
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Supplies(Vec<CrateStack>);

impl Supplies {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, value: CrateStack) {
        self.0.push(value)
    }

    fn max_size(&self) -> usize {
        self.0.iter().map(|stack| stack.len()).max().unwrap_or(0)
    }

    fn apply_move(&mut self, r#move: Move) {
        // Move the crates
        for _ in 0..r#move.count {
            let item = self
                .0
                .get_mut(r#move.from - 1)
                .expect("Not enough crate stacks")
                .pop()
                .expect("Not enough items on the crate stack");

            self.0
                .get_mut(r#move.to - 1)
                .expect("Not enough crate stacks")
                .push(item);
        }
    }

    fn top_crates(&self) -> String {
        self.0
            .iter()
            .map(|stack| stack.top().map_or("".to_string(), |cr| cr.0.to_string()))
            .collect()
    }
}

impl FromIterator<CrateStack> for Supplies {
    fn from_iter<T: IntoIterator<Item = CrateStack>>(iter: T) -> Self {
        Supplies(iter.into_iter().collect())
    }
}

impl IntoIterator for Supplies {
    type Item = CrateStack;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct Day05;

impl Day for Day05 {
    fn identifier(&self) -> &'static str {
        "05"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1(&input));
        println!("Part 2: TODO");
    }
}

fn part_1(input: &str) -> String {
    let mut supplies = parse_supplies(input);

    let max_size = supplies.max_size();
    let instruction_start_row = max_size + 2;

    // Parse the moves and apply them
    input
        .lines()
        .skip(instruction_start_row)
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Move>().unwrap())
        .for_each(|r#move| supplies.apply_move(r#move));

    supplies.top_crates()
}

fn parse_supplies(input: &str) -> Supplies {
    let lines: Vec<&str> = input.lines().collect();

    let mut supplies = Supplies::new();

    // Parse each stack one-by-one
    'outer: for stack_index in 0usize.. {
        // The column in the input that the stack occupies
        let col_index = 1 + stack_index * 4;

        let mut rev_container_names: Vec<char> = Vec::new();

        for row_index in 0usize.. {
            let row = lines.get(row_index).unwrap();
            // Get the next character to parse in the column
            // If the input ends in this column, we reached the last stack
            let Some(col) = row.chars().nth(col_index) else {
                break 'outer;
            };

            match col {
                // The empty "top" of the container, just skip it
                ' ' => continue,
                // A new container, add it to the stack
                'A'..='Z' => rev_container_names.push(col),
                // The end of the stack, go to the next stack
                '1'..='9' => break,
                _ => panic!("Unexpected token on a container stack: '{col}'"),
            }
        }

        // Assemble the container stack
        let stack: CrateStack = rev_container_names.into_iter().rev().collect();

        supplies.push(stack);
    }

    supplies
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn should_parse_container_stacks() {
        let expected = Supplies::from_iter([
            CrateStack::from_iter(['Z', 'N']),
            CrateStack::from_iter(['M', 'C', 'D']),
            CrateStack::from_iter(['P']),
        ]);
        let actual = parse_supplies(EXAMPLE_INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_calculate_part_1_solution() {
        let expected = "CMZ".to_string();
        let actual = part_1(EXAMPLE_INPUT);

        assert_eq!(actual, expected);
    }
}
