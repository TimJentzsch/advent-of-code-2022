use std::ops::RangeInclusive;

use crate::utils::Day;

trait RangeExt {
    fn contains_all(&self, other: &Self) -> bool;

    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeExt for RangeInclusive<usize> {
    fn contains_all(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
            || self.start() >= other.end() && self.end() <= other.start()
    }
}

pub struct Day04;

impl Day for Day04 {
    fn identifier(&self) -> &'static str {
        "04"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1(&input));
        println!("Part 2: {}", part_2(&input));
    }
}

fn parse_range(input: &str) -> RangeInclusive<usize> {
    input
        .split_once('-')
        .map(|(start_str, end_str)| start_str.parse().unwrap()..=end_str.parse().unwrap())
        .unwrap()
}

fn parse_ranges(line: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    line.split_once(',')
        .map(|(first_str, second_str)| (parse_range(first_str), parse_range(second_str)))
        .unwrap()
}

fn part_1(input: &str) -> usize {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(parse_ranges)
        .filter(|(first, second)| first.contains_all(second) || second.contains_all(first))
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(parse_ranges)
        .filter(|(first, second)| first.overlaps(second))
        .count()
}
