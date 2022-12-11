use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

use crate::utils::Day;

type MonkeyIndex = usize;
type WorryLevel = u64;

/// Compute the greatest common divisor of `a` and `b`.
fn gcd(a: WorryLevel, b: WorryLevel) -> WorryLevel {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let (mut a, mut b) = (a, b);

    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

/// Compute the least common denominator of `a` and `b`.
fn lcd(a: WorryLevel, b: WorryLevel) -> WorryLevel {
    (a / gcd(a, b)) * b
}

/// Compute the least common denominator of all provided numbers.
fn lcd_many<I>(levels: I) -> WorryLevel
where
    I: IntoIterator<Item = WorryLevel>,
{
    levels.into_iter().fold(1, lcd)
}

trait SliceExt<T> {
    fn split_3_at_mut(&mut self, mid: usize) -> (&mut [T], &mut T, &mut [T]);
}

impl<T> SliceExt<T> for [T] {
    fn split_3_at_mut(&mut self, mid: usize) -> (&mut [T], &mut T, &mut [T]) {
        assert!(
            !self.is_empty(),
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    items: Vec<WorryLevel>,
    operation: Operation,
    test: Test,
}

struct MonkeyInTheMiddle {
    monkeys: Vec<Monkey>,
    inspect_count: Vec<u64>,
}

impl MonkeyInTheMiddle {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let count = monkeys.len();

        Self {
            monkeys,
            inspect_count: (0..count).map(|_| 0).collect(),
        }
    }
}

impl MonkeyInTheMiddle {
    fn round(&mut self, enable_worry_reduction: bool) {
        // Get the least common denominator of the divisor tests and 3 (for the worry decrease)
        // This allows us to safely cap by this value without changing the tests
        let worry_lcd = lcd(
            lcd_many(self.monkeys.iter().map(|monkey| monkey.test.divisible_by)),
            3,
        );

        for monkey_idx in 0..self.monkeys.len() {
            let (before, monkey, after) = self.monkeys.split_3_at_mut(monkey_idx);

            for item in monkey.items.drain(..) {
                // Monkey inspects item
                let mut worry = monkey.operation.evaluate(item);
                self.inspect_count[monkey_idx] += 1;

                // Worry level decreases
                if enable_worry_reduction {
                    worry /= 3;
                }

                // Make sure value doesn't grow too much
                worry %= worry_lcd;

                // Monkey throws item
                let next_idx = monkey.test.get_next_monkey(worry);

                let next_monkey = match next_idx.cmp(&monkey_idx) {
                    std::cmp::Ordering::Less => before.get_mut(next_idx).unwrap(),
                    std::cmp::Ordering::Equal => panic!("Next monkey is same as current monkey"),
                    std::cmp::Ordering::Greater => {
                        after.get_mut(next_idx - monkey_idx - 1).unwrap()
                    }
                };

                next_monkey.items.push(worry);
            }
        }
    }

    fn monkey_business_level(&self) -> u64 {
        let mut counts: Vec<u64> = self.inspect_count.clone();
        counts.sort_unstable();
        counts.iter().rev().take(2).product()
    }
}

fn parse_ws(input: &str) -> IResult<&str, ()> {
    map(take_while1(|c: char| c.is_ascii_whitespace()), |_| ())(input)
}

fn parse_worry_level(input: &str) -> IResult<&str, WorryLevel> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |num: &str| {
        num.parse::<WorryLevel>()
    })(input)
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<WorryLevel>> {
    preceded(
        tag("Starting items: "),
        separated_list0(tag(", "), parse_worry_level),
    )(input)
}

fn parse_val(input: &str) -> IResult<&str, Val> {
    let parse_old = map(tag("old"), |_| Val::Old);
    let parse_num = map(parse_worry_level, Val::Num);

    alt((parse_old, parse_num))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let parse_add = map(
        separated_pair(parse_val, tag(" + "), parse_val),
        |(lhs, rhs)| Operation::Add(lhs, rhs),
    );
    let parse_mul = map(
        separated_pair(parse_val, tag(" * "), parse_val),
        |(lhs, rhs)| Operation::Mul(lhs, rhs),
    );

    preceded(tag("Operation: new = "), alt((parse_add, parse_mul)))(input)
}

fn parse_monkey_index(input: &str) -> IResult<&str, MonkeyIndex> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |num: &str| {
        num.parse::<MonkeyIndex>()
    })(input)
}

fn parse_throw_to(input: &str) -> IResult<&str, MonkeyIndex> {
    preceded(tag("throw to monkey "), parse_monkey_index)(input)
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let parse_divisible_by = preceded(tag("Test: divisible by "), parse_worry_level);
    let parse_true = preceded(tag("If true: "), parse_throw_to);
    let parse_false = preceded(tag("If false: "), parse_throw_to);

    map(
        tuple((
            parse_divisible_by,
            parse_ws,
            parse_true,
            parse_ws,
            parse_false,
        )),
        |(divisible_by, _, if_true, _, if_false)| Test {
            divisible_by,
            if_true,
            if_false,
        },
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let parse_monkey_idx = delimited(tag("Monkey "), parse_monkey_index, tag(":"));

    map(
        tuple((
            parse_monkey_idx,
            parse_ws,
            parse_starting_items,
            parse_ws,
            parse_operation,
            parse_ws,
            parse_test,
        )),
        |(_, _, items, _, operation, _, test)| Monkey {
            items,
            operation,
            test,
        },
    )(input)
}

fn parse_monkey_in_the_middle(input: &str) -> IResult<&str, MonkeyInTheMiddle> {
    map(separated_list0(parse_ws, parse_monkey), |monkeys| {
        MonkeyInTheMiddle::new(monkeys)
    })(input.trim())
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

fn part_1(input: &str) -> WorryLevel {
    let (_, mut monkey_in_the_middle) = parse_monkey_in_the_middle(input).unwrap();

    for _ in 0..20 {
        monkey_in_the_middle.round(true);
    }

    monkey_in_the_middle.monkey_business_level()
}

fn part_2(input: &str) -> WorryLevel {
    let (_, mut monkey_in_the_middle) = parse_monkey_in_the_middle(input).unwrap();

    for _ in 0..10000 {
        monkey_in_the_middle.round(false);
    }

    monkey_in_the_middle.monkey_business_level()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
";

    #[test]
    fn should_calculate_gcd() {
        assert_eq!(gcd(143, 65), 13);
    }

    #[test]
    fn should_calculate_lcd() {
        assert_eq!(lcd(12, 18), 36);
    }

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
    fn should_parse_monkey() {
        let monkey_str = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3";

        let expected = Monkey {
            items: vec![79, 98],
            operation: Operation::Mul(Val::Old, Val::Num(19)),
            test: Test {
                divisible_by: 23,
                if_true: 2,
                if_false: 3,
            },
        };

        let actual = parse_monkey(monkey_str);

        assert_eq!(actual, Ok(("", expected)));
    }

    #[test]
    fn should_calculate_part_1_solution() {
        let actual = part_1(EXAMPLE_INPUT);

        assert_eq!(actual, 10605);
    }

    #[test]
    fn should_calculate_part_2_solution() {
        let actual = part_2(EXAMPLE_INPUT);

        assert_eq!(actual, 2713310158);
    }
}
