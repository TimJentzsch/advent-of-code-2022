use std::collections::HashMap;

use crate::utils::Day;

type MonkeyIndex = usize;
type WorryLevel = u64;

trait SliceExt<T> {
    fn split_3_at_mut(&mut self, mid: usize) -> (&mut [T], &mut T, &mut [T]);
}

impl<T> SliceExt<T> for [T] {
    fn split_3_at_mut(&mut self, mid: usize) -> (&mut [T], &mut T, &mut [T]) {
        assert!(
            self.is_empty(),
            "The slice must contain at least one element"
        );
        assert!(
            mid < self.len(),
            "The mid index must be contained in the slice"
        );

        let (first, second) = self.split_at_mut(mid);
        let (mid, third) = second.split_at_mut(1);
        (first, mid.get_mut(0).unwrap(), third)
    }
}

enum Val {
    Old,
    Num(WorryLevel),
}

impl Val {
    fn evaluate(&self, old: WorryLevel) -> WorryLevel {
        match self {
            Val::Old => old,
            Val::Num(num) => *num,
        }
    }
}

enum Operation {
    Add(Val, Val),
    Mul(Val, Val),
}

impl Operation {
    fn evaluate(&self, old: WorryLevel) -> WorryLevel {
        match self {
            Operation::Add(lhs, rhs) => lhs.evaluate(old) + rhs.evaluate(old),
            Operation::Mul(lhs, rhs) => lhs.evaluate(old) * rhs.evaluate(old),
        }
    }
}

struct Test {
    divisible_by: WorryLevel,
    if_true: MonkeyIndex,
    if_false: MonkeyIndex,
}

impl Test {
    fn get_next_monkey(&self, worry_level: WorryLevel) -> MonkeyIndex {
        if worry_level % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

struct Monkey {
    items: Vec<WorryLevel>,
    operation: Operation,
    test: Test,
}

struct MonkeyInTheMiddle<const M: usize> {
    monkeys: [Monkey; M],
    inspect_count: [u64; M],
}

impl<const M: usize> MonkeyInTheMiddle<M> {
    fn round(&mut self) {
        for monkey_idx in 0..self.monkeys.len() {
            let (before, monkey, after) = self.monkeys.split_3_at_mut(monkey_idx);

            for item in monkey.items.drain(..) {
                // Monkey inspects item
                let mut item = monkey.operation.evaluate(item);
                self.inspect_count[monkey_idx] += 1;

                // Worry level decreases
                item /= 3;

                // Monkey throws item
                let next_idx = monkey.test.get_next_monkey(item);

                let next_monkey = match next_idx.cmp(&monkey_idx) {
                    std::cmp::Ordering::Less => before.get_mut(next_idx).unwrap(),
                    std::cmp::Ordering::Equal => after.get_mut(next_idx - monkey_idx - 1).unwrap(),
                    std::cmp::Ordering::Greater => panic!("Next monkey is same as current monkey"),
                };

                next_monkey.items.push(item);
            }
        }
    }

    fn monkey_business_level(&self) -> u64 {
        let mut counts: Vec<u64> = self.inspect_count.iter().copied().collect();
        counts.sort_unstable();
        counts.iter().rev().take(2).sum()
    }
}

pub struct Day11;

impl Day for Day11 {
    fn identifier(&self) -> &'static str {
        "11"
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
    fn should_split_3_at_mut_middle() {
        let slice = &mut [0, 1, 2, 3, 4, 5];
        let (before, mid, after) = slice.split_3_at_mut(3);

        assert_eq!(before.iter().copied().collect::<Vec<_>>(), vec![0, 1, 2]);
        assert_eq!(*mid, 3);
        assert_eq!(after.iter().copied().collect::<Vec<_>>(), vec![4, 5]);
    }

    #[test]
    fn should_split_3_at_mut_start() {
        let slice = &mut [0, 1, 2, 3, 4, 5];
        let (before, mid, after) = slice.split_3_at_mut(0);

        assert_eq!(before.iter().copied().collect::<Vec<_>>(), vec![]);
        assert_eq!(*mid, 0);
        assert_eq!(
            after.iter().copied().collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn should_split_3_at_mut_end() {
        let slice = &mut [0, 1, 2, 3, 4, 5];
        let (before, mid, after) = slice.split_3_at_mut(5);

        assert_eq!(
            before.iter().copied().collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(*mid, 5);
        assert_eq!(after.iter().copied().collect::<Vec<_>>(), vec![]);
    }

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
