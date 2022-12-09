use crate::utils::Day;

pub struct DayXX;

impl Day for DayXX {
    fn identifier(&self) -> &'static str {
        "XX"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1(&input));
        println!("Part 2: {}", part_2(&input));
    }
}

fn part_1(_input: &str) -> usize {
    0
}

fn part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "";

    #[test]
    fn should_calculate_part_1_solution() {
        let actual = part_1(EXAMPLE_INPUT);

        assert_eq!(actual, 0);
    }

    #[test]
    fn should_calculate_part_2_solution() {
        let actual = part_2(EXAMPLE_INPUT);

        assert_eq!(actual, 0);
    }
}
