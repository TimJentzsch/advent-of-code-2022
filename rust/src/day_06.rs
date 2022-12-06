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

fn find_marker(input: &str, marker_size: usize) -> usize {
    let chars: Vec<_> = input.trim().chars().collect();

    let sequence_count = chars
        .windows(marker_size)
        .take_while(|sequence| HashSet::<_>::from_iter(sequence.iter()).len() < marker_size)
        .count();

    // Add the sequence length to get the count of characters
    sequence_count + marker_size
}

fn part_1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part_2(input: &str) -> usize {
    find_marker(input, 14)
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

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn should_calculate_part_2_solution(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part_2(input), expected);
    }
}
