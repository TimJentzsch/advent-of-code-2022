use std::collections::HashSet;

use crate::utils::Day;

pub struct Day06;

impl Day for Day06 {
    fn identifier(&self) -> &'static str {
        "06"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1(&input));
        println!("Part 2: {}", part_2(&input));
    }
}

fn part_1(input: &str) -> usize {
    let chars: Vec<_> = input.trim().chars().collect();

    let sequence_length = 4;

    let sequence_count = chars
        .windows(sequence_length)
        .take_while(|sequence| {
            HashSet::<char>::from_iter(sequence.iter().copied()).len() < sequence_length
        })
        .count();

    // Add the sequence length to get the count of characters
    sequence_count + sequence_length
}

fn part_2(_input: &str) -> String {
    "TODO".to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn should_calculate_part_1_solution(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part_1(input), expected);
    }
}
