use std::{fmt::Display, str::FromStr};

use crate::utils::Day;

#[derive(Debug)]
struct ParsingError;

#[derive(Debug, PartialEq, Eq)]
enum ExecutionError {
    EndOfProgram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Noop,
    AddX(i64),
}

impl Instruction {
    fn duration(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        let instruction = match tokens.next().expect("Expected command identifier") {
            "noop" => Self::Noop,
            "addx" => {
                let val = tokens
                    .next()
                    .expect("Expected value for addx")
                    .parse()
                    .expect("Expected number");
                Self::AddX(val)
            }
            _ => return Err(ParsingError),
        };

        Ok(instruction)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Program(Vec<Instruction>);

impl Program {}

impl FromStr for Program {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Result<Vec<Instruction>, ParsingError> =
            s.lines().map(|line| line.parse::<Instruction>()).collect();

        Ok(Self(instructions?))
    }
}

#[derive(Debug)]
struct ClockCircuit<const W: usize, const H: usize> {
    program: Program,
    program_counter: usize,
    cycle_counter: usize,
    x: i64,
    buffer: Option<(Instruction, usize)>,
    crt: [[bool; W]; H],
}

impl<const W: usize, const H: usize> ClockCircuit<W, H> {
    fn new(program: Program) -> Self {
        Self {
            program,
            program_counter: 0,
            cycle_counter: 0,
            x: 1,
            buffer: None,
            crt: [[false; W]; H],
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => (),
            Instruction::AddX(val) => self.x += val,
        };

        self.program_counter += 1;
    }

    fn crt_position(&self) -> (usize, usize) {
        let row = (self.cycle_counter - 1) / W;
        let col = (self.cycle_counter - 1) % W;

        (row, col)
    }

    /// The start of a cycle tick.
    ///
    /// Starts a new instruction if none is currently running.
    fn tick_start(&mut self) -> Result<(), ExecutionError> {
        self.cycle_counter += 1;

        if self.buffer.is_none() {
            let next_instruction =
                if let Some(instruction) = self.program.0.get(self.program_counter) {
                    instruction
                } else {
                    return Err(ExecutionError::EndOfProgram);
                };

            self.buffer = Some((*next_instruction, next_instruction.duration()));
        }

        Ok(())
    }

    /// The middle of the cycle tick.
    ///
    /// Draws a pixel on the CRT and returns the value of x.
    fn tick_middle(&mut self) -> Result<i64, ExecutionError> {
        let (row, col) = self.crt_position();

        // Only draw if the pixel is within the sprite
        if self.x - 1 <= col as i64 && self.x + 1 >= col as i64 {
            self.crt[row][col] = true;
        }

        Ok(self.x)
    }

    /// The end of a cycle tick.
    ///
    /// Processes instructions in the buffer and executes them once they are done.
    fn tick_end(&mut self) -> Result<(), ExecutionError> {
        if let Some((instruction, remaining)) = self.buffer {
            let remaining = remaining.wrapping_sub(1);

            if remaining == 0 {
                self.execute(&instruction);
                self.buffer = None;
            } else {
                self.buffer = Some((instruction, remaining));
            }
        }

        Ok(())
    }

    /// Executes a single tick and returns the value of x.
    fn tick(&mut self) -> Result<i64, ExecutionError> {
        self.tick_start()?;
        let x = self.tick_middle()?;
        self.tick_end()?;

        Ok(x)
    }
}

impl<const W: usize, const H: usize> Display for ClockCircuit<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display: String = self
            .crt
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&pixel| if pixel { "#" } else { "." })
                    .collect::<String>()
            })
            .intersperse("\n".to_string())
            .collect();

        write!(f, "{display}")
    }
}

pub struct Day10;

impl Day for Day10 {
    fn identifier(&self) -> &'static str {
        "10"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1(&input));
        println!("Part 2:\n{}", part_2(&input));
    }
}

fn part_1(input: &str) -> i64 {
    let mut signal_strength: i64 = 0;

    let program: Program = input.parse().unwrap();
    let mut clock_circuit = ClockCircuit::<40, 6>::new(program);

    let mut x = 1;

    for _ in 0..20 {
        if let Ok(new_x) = clock_circuit.tick() {
            x = new_x;
        } else {
            break;
        }
    }

    signal_strength += clock_circuit.cycle_counter as i64 * x;

    'outer: loop {
        for _ in 0..40 {
            if let Ok(new_x) = clock_circuit.tick() {
                x = new_x;
            } else {
                break 'outer;
            }
        }

        signal_strength += clock_circuit.cycle_counter as i64 * x;
    }

    signal_strength
}

fn part_2(input: &str) -> String {
    let program: Program = input.parse().unwrap();
    let mut clock_circuit = ClockCircuit::<40, 6>::new(program);

    loop {
        if clock_circuit.tick().is_err() {
            break;
        }
    }

    format!("{clock_circuit}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn should_calculate_part_1_solution() {
        let actual = part_1(EXAMPLE_INPUT);

        assert_eq!(actual, 13140);
    }

    #[test]
    fn should_calculate_part_2_solution() {
        let actual = part_2(EXAMPLE_INPUT);

        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        assert_eq!(actual, expected.to_string());
    }
}
