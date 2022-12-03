use std::collections::HashSet;

use crate::utils::Day;

type Priority = u32;

trait Item {
    fn priority(&self) -> Priority;
}

impl Item for char {
    fn priority(&self) -> Priority {
        match self {
            'a'..='z' => *self as u32 - 'a' as u32 + 1,
            'A'..='Z' => *self as u32 - 'A' as u32 + 27,
            _ => panic!("Invalid item '{self}'"),
        }
    }
}

trait ItemSet {
    fn unique_items(&self) -> HashSet<char>;
}

impl ItemSet for &str {
    fn unique_items(&self) -> HashSet<char> {
        HashSet::from_iter(self.chars())
    }
}

pub struct Day03;

impl Day for Day03 {
    fn identifier(&self) -> &'static str {
        "03"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1(input.clone()));
        println!("Part 2: {}", part_2(input));
    }
}

fn part_1(input: String) -> Priority {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Get the two item sets
            let (first, second) = line.split_at(line.len() / 2);

            // Calculate the priority of duplicate items
            first
                .unique_items()
                .intersection(&second.unique_items())
                .map(|item| item.priority())
                .sum::<Priority>()
        })
        .sum()
}

fn part_2(input: String) -> Priority {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        // Divide into packs of 3
        .array_chunks::<3>()
        .map(|[first, second, third]| {
            // Get common items of first and second inventory
            let first_and_second: HashSet<char> = first
                .unique_items()
                .intersection(&second.unique_items())
                .into_iter()
                .copied()
                .collect();

            // Get common items with third inventory
            first_and_second
                .intersection(&third.unique_items())
                // Calculate priorities
                .map(|item| item.priority())
                .sum::<Priority>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[rstest]
    #[case('a', 1)]
    #[case('z', 26)]
    #[case('A', 27)]
    #[case('Z', 52)]
    fn item_priorities(#[case] item: char, #[case] expected: Priority) {
        let actual = item.priority();

        assert_eq!(
            actual, expected,
            "'{item}' should be {expected}, but is {actual}"
        );
    }

    #[test]
    fn example_for_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT.to_string()), 157);
    }

    #[test]
    fn example_for_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT.to_string()), 70);
    }
}
