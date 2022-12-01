use crate::utils::Day;

pub struct Day01;

impl Day for Day01 {
    fn identifier(&self) -> &'static str {
        "01"
    }

    fn run(&self) {
        let input = self.get_input();

        let mut calories: Vec<u32> = input
            .split("\n\n")
            .map(|chunk| chunk.lines().map(|line| line.parse::<u32>().unwrap()).sum())
            .collect();

        calories.sort_unstable();

        println!("Part 1: {}", calories.last().unwrap());
        println!("Part 2: {}", calories.iter().rev().take(3).sum::<u32>())
    }
}
