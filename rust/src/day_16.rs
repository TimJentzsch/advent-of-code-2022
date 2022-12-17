use std::{
    collections::{BinaryHeap, HashMap},
    slice::Iter,
};

use crate::utils::Day;

type Pressure = u16;

type Minute = u8;

type ValveIndex = usize;

/// The two letter names for each valve.
#[derive(Debug, PartialEq, Eq, Clone)]
struct ValveName(String);

/// A map from the valve names to their index.
type ValveIndexMap = HashMap<ValveName, ValveIndex>;

/// A map of the flow rate for each valve.
#[derive(Debug, PartialEq, Eq, Clone)]
struct FlowRates<const N: usize>([Pressure; N]);

/// A map of the adjacent valves for each valve.
#[derive(Debug, PartialEq, Eq, Clone)]
struct AdjacentValves<const N: usize>([Vec<ValveIndex>; N]);

/// A list of the currently open valves.
#[derive(Debug, PartialEq, Eq, Clone)]
struct OpenValves(Vec<ValveIndex>);

impl OpenValves {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn contains(&self, valve: &ValveIndex) -> bool {
        self.0.contains(valve)
    }

    fn open(&mut self, valve: ValveIndex) {
        self.0.push(valve);
    }

    fn iter(&self) -> Iter<'_, ValveIndex> {
        self.0.iter()
    }
}

/// A list of valves that are still reachable, and the time it takes to reach them.
#[derive(Debug, PartialEq, Eq, Clone)]
struct ReachableValves(Vec<(ValveIndex, Minute)>);

impl ReachableValves {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn contains(&self, valve: &ValveIndex) -> bool {
        for (cur_valve, _) in &self.0 {
            if valve == cur_valve {
                return true;
            }
        }

        false
    }

    fn iter(&self) -> Iter<'_, (ValveIndex, Minute)> {
        self.0.iter()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl FromIterator<(ValveIndex, Minute)> for ReachableValves {
    fn from_iter<T: IntoIterator<Item = (ValveIndex, Minute)>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GameInfo<const N: usize> {
    flow_rates: FlowRates<N>,
    adjacent_valves: AdjacentValves<N>,
    total_minutes: Minute,
}

impl<const N: usize> GameInfo<N> {
    fn move_time(&self, from: ValveIndex, to: ValveIndex) -> Minute {
        todo!("Implement move time algorithm")
    }
}

#[derive(Debug, Clone)]
struct GameState<const N: usize> {
    cur_valve: ValveIndex,
    open_valves: OpenValves,
    cur_pressure_release: Pressure,
    cur_minute: Minute,

    /// The valves that are still reachable within the remaining time and the time it takes to reach them.
    reachable_valves: ReachableValves,

    /// An upper bound for the pressure that can still be released.
    heuristic: Pressure,
}

impl<const N: usize> GameState<N> {
    fn start(info: &GameInfo<N>) -> Self {
        let open_valves = OpenValves::new();
        let reachable_valves = ReachableValves::new();

        Self {
            cur_minute: 0,
            cur_pressure_release: 0,
            cur_valve: 0,
            heuristic: Self::calculate_heuristic(
                info.total_minutes,
                &open_valves,
                &reachable_valves,
                info,
            ),
            open_valves,
            reachable_valves,
        }
    }

    fn next_state(
        &self,
        next_valve: ValveIndex,
        move_and_open_time: Minute,
        info: &GameInfo<N>,
    ) -> Self {
        // The time passes while we move to the next valve and open it
        let cur_minute = self.cur_minute + move_and_open_time;

        // Release pressure from the open valves during the time
        let cur_pressure_release =
            self.cur_pressure_release + self.released_pressure(move_and_open_time, info);

        // Move to the next valve
        let cur_valve = next_valve;

        // Open the next valve
        let mut open_valves = self.open_valves.clone();
        open_valves.open(next_valve);

        let remaining_time = info.total_minutes - cur_minute;

        // Compute the reachable valves from this point
        let reachable_valves = (0..N)
            .filter(|valve| !open_valves.contains(valve))
            .filter_map(|valve| {
                let time = info.move_time(cur_valve, valve) + 1;

                if time <= remaining_time {
                    Some((valve, time))
                } else {
                    None
                }
            })
            .collect();

        // Pre-compute heuristic
        let heuristic =
            Self::calculate_heuristic(remaining_time, &open_valves, &reachable_valves, info);

        Self {
            cur_minute,
            cur_pressure_release,
            cur_valve,
            open_valves,
            reachable_valves,
            heuristic,
        }
    }

    fn calculate_heuristic(
        remaining_time: Minute,
        open_valves: &OpenValves,
        reachable_valves: &ReachableValves,
        info: &GameInfo<N>,
    ) -> Pressure {
        // The open valve can release the remaining pressure
        let open_valve_value = open_valves
            .iter()
            .map(|&valve| {
                let flow_rate = info.flow_rates.0[valve];
                flow_rate * remaining_time as Pressure
            })
            .sum::<Pressure>();

        // We can go to the closed valves and open them to release more pressure
        // This is an upper bound, as we cannot go to multiple valves "at the same time"
        let closed_valve_value = reachable_valves
            .iter()
            .map(|&(valve, time)| {
                // Open the valve in the remaining time
                let flow_rate = info.flow_rates.0[valve];
                remaining_time.saturating_sub(time) as Pressure * flow_rate
            })
            .sum::<Pressure>();

        open_valve_value + closed_valve_value
    }

    fn released_pressure(&self, time: Minute, info: &GameInfo<N>) -> Pressure {
        self.open_valves
            .iter()
            .map(|&valve| info.flow_rates.0[valve] * time as Pressure)
            .sum()
    }

    fn remaining_time(&self, info: &GameInfo<N>) -> Minute {
        info.total_minutes - self.cur_minute
    }

    fn expand(&mut self, info: &GameInfo<N>) -> Vec<GameState<N>> {
        let remaining_time = self.remaining_time(info);

        // We can move to any closed valve and open it (if there is enough time)
        (0..N)
            // Closed valves
            .filter(|valve| !self.open_valves.contains(valve))
            .filter_map(|next_valve| {
                // We need time to move to the valve and then 1 minute to open it
                let move_and_open_time = info.move_time(self.cur_valve, next_valve) + 1;

                if move_and_open_time <= remaining_time {
                    Some(self.next_state(next_valve, move_and_open_time, info))
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_leaf(&self) -> bool {
        self.reachable_valves.is_empty()
    }

    /// An upper bound for the total pressure released of this state.
    ///
    /// This is equal to the actual total pressure released if `.is_leaf()` is `true`.
    fn score(&self) -> Pressure {
        self.cur_pressure_release + self.heuristic
    }
}

impl<const N: usize> PartialEq<GameState<N>> for GameState<N> {
    fn eq(&self, other: &GameState<N>) -> bool {
        self.score().eq(&other.score())
    }
}

impl<const N: usize> Eq for GameState<N> {}

impl<const N: usize> PartialOrd<GameState<N>> for GameState<N> {
    fn partial_cmp(&self, other: &GameState<N>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for GameState<N> {
    fn cmp(&self, other: &GameState<N>) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PressureReleaseSearch<const N: usize> {
    info: GameInfo<N>,
}

impl<const N: usize> PressureReleaseSearch<N> {
    fn new(info: GameInfo<N>) -> Self {
        Self { info }
    }

    fn search(&self) -> GameState<N> {
        // Do a modified A* search
        let mut open_set: BinaryHeap<GameState<N>> = BinaryHeap::new();
        open_set.push(GameState::<N>::start(&self.info));

        while let Some(mut current) = open_set.pop() {
            if current.is_leaf() {
                return current;
            }

            open_set.extend(current.expand(&self.info).into_iter());
        }

        panic!("Unexpected end of search without result!");
    }

    fn part_1(&self) -> Pressure {
        self.search().score()
    }
}

pub struct Day16;

impl Day for Day16 {
    fn identifier(&self) -> &'static str {
        "16"
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
