use std::{
    collections::{BinaryHeap, HashMap},
    slice::Iter,
    str::FromStr,
};

use crate::utils::Day;

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

type Pressure = u16;

type Minute = u8;

type ValveIndex = usize;

struct ParsedValve {
    name: String,
    flow_rate: Pressure,
    adjacent_valves: Vec<String>,
}

impl FromStr for ParsedValve {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = &mut s.split_ascii_whitespace();

        let Some(name) = tokens.nth(1) else {
            return Err(ParseError);
        };

        let Ok(flow_rate) = (if let Some(rate_str) = tokens.nth(2) {
            let Some((_, end_str)) = rate_str.split_once('=') else {
                return Err(ParseError);
            };

            end_str[..(end_str.len() - 1)].parse::<Pressure>()
        } else {
            return Err(ParseError);
        }) else {
            return Err(ParseError)
        };

        let adjacent_valves: Vec<String> = tokens
            .skip(4)
            .collect::<String>()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        Ok(Self {
            name: name.to_string(),
            flow_rate,
            adjacent_valves,
        })
    }
}

impl PartialEq for ParsedValve {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for ParsedValve {}

impl PartialOrd for ParsedValve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ParsedValve {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

/// A map of the flow rate for each valve.
#[derive(Debug, PartialEq, Eq, Clone)]
struct FlowRates<const N: usize>([Pressure; N]);

impl<const N: usize> TryFrom<Vec<Pressure>> for FlowRates<N> {
    type Error = Vec<Pressure>;

    fn try_from(value: Vec<Pressure>) -> Result<Self, Self::Error> {
        let flow_rates: [Pressure; N] = value.try_into()?;
        Ok(Self(flow_rates))
    }
}

/// A map of the adjacent valves for each valve.
#[derive(Debug, PartialEq, Eq, Clone)]
struct AdjacentValves<const N: usize>([Vec<ValveIndex>; N]);

impl<const N: usize> TryFrom<Vec<Vec<ValveIndex>>> for AdjacentValves<N> {
    type Error = Vec<Vec<ValveIndex>>;

    fn try_from(value: Vec<Vec<ValveIndex>>) -> Result<Self, Self::Error> {
        let adjacent_valves: [Vec<ValveIndex>; N] = value.try_into()?;
        Ok(Self(adjacent_valves))
    }
}

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

type MoveMap = HashMap<(ValveIndex, ValveIndex), Minute>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct GameInfo<const N: usize> {
    flow_rates: FlowRates<N>,
    adjacent_valves: AdjacentValves<N>,
    total_minutes: Minute,
}

impl<const N: usize> GameInfo<N> {
    fn move_time(&self, from: ValveIndex, to: ValveIndex) -> Minute {
        // TODO: Make this more efficient
        let mut reachable = vec![from];

        for move_time in 0..N {
            if reachable.contains(&to) {
                return move_time as Minute;
            }

            reachable = reachable
                .into_iter()
                .flat_map(|valve| &self.adjacent_valves.0[valve])
                .copied()
                .collect();
        }

        panic!("{to} is not reachable from {from}");
    }

    fn flow_rate(&self, valve: ValveIndex) -> Pressure {
        self.flow_rates.0[valve]
    }

    fn compute_move_map(&self) -> MoveMap {
        let interesting_valves: Vec<_> =
            (0..N).filter(|&valve| self.flow_rate(valve) > 0).collect();

        interesting_valves
            .iter()
            .chain(vec![0].iter())
            .flat_map(|&from| {
                interesting_valves
                    .iter()
                    .map(move |&to| ((from, to), self.move_time(from, to)))
            })
            .collect()
    }
}

impl<const N: usize> FromStr for GameInfo<N> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed_valves = s
            .trim()
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<ParsedValve>, Self::Err>>()?;

        parsed_valves.sort();

        let index_map: HashMap<&str, ValveIndex> = parsed_valves
            .iter()
            .enumerate()
            .map(|(index, parsed_valve)| (parsed_valve.name.as_str(), index))
            .collect();

        let adjacent_valves: Vec<_> = parsed_valves
            .iter()
            .map(|valve| {
                valve
                    .adjacent_valves
                    .iter()
                    .map(|name| {
                        *index_map
                            .get(name.as_str())
                            .unwrap_or_else(|| panic!("Can't map {name} to a valve index"))
                    })
                    .collect::<Vec<ValveIndex>>()
            })
            .collect();
        let adjacent_valves = adjacent_valves.try_into().map_err(|_| ParseError)?;

        let flow_rates: Vec<_> = parsed_valves.iter().map(|valve| valve.flow_rate).collect();
        let flow_rates = flow_rates.try_into().map_err(|_| ParseError)?;

        Ok(Self {
            adjacent_valves,
            flow_rates,
            total_minutes: 30,
        })
    }
}

#[derive(Debug, Clone)]
struct GameState<const N: usize> {
    open_valves: OpenValves,
    cur_pressure_release: Pressure,
    cur_minute: Minute,

    /// The valves that are still reachable within the remaining time and the time it takes to reach them.
    reachable_valves: ReachableValves,

    /// An upper bound for the pressure that can still be released.
    heuristic: Pressure,
}

impl<const N: usize> GameState<N> {
    fn start(info: &GameInfo<N>, move_map: &MoveMap) -> Self {
        let open_valves = OpenValves::new();
        let reachable_valves =
            Self::calculate_reachable_valves(info.total_minutes, 0, &open_valves, info, move_map);

        Self {
            cur_minute: 0,
            cur_pressure_release: 0,
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
        move_map: &MoveMap,
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
        let reachable_valves = Self::calculate_reachable_valves(
            remaining_time,
            cur_valve,
            &open_valves,
            info,
            move_map,
        );

        // Pre-compute heuristic
        let heuristic =
            Self::calculate_heuristic(remaining_time, &open_valves, &reachable_valves, info);

        Self {
            cur_minute,
            cur_pressure_release,
            open_valves,
            reachable_valves,
            heuristic,
        }
    }

    fn calculate_reachable_valves(
        remaining_time: Minute,
        cur_valve: ValveIndex,
        open_valves: &OpenValves,
        info: &GameInfo<N>,
        move_map: &MoveMap,
    ) -> ReachableValves {
        (0..N)
            // Only consider valves that are not opened yet and that have any flow
            .filter(|valve| !open_valves.contains(valve) && info.flow_rate(*valve) != 0)
            // Only consider valves that can still be reached and opened in the remaining time
            .filter_map(|valve| {
                let time = move_map.get(&(cur_valve, valve)).unwrap() + 1;

                if time <= remaining_time {
                    Some((valve, time))
                } else {
                    None
                }
            })
            .collect()
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
            .map(|&valve| info.flow_rate(valve) * remaining_time as Pressure)
            .sum::<Pressure>();

        // We can go to the closed valves and open them to release more pressure
        // This is an upper bound, as we cannot go to multiple valves "at the same time"
        let closed_valve_value = reachable_valves
            .iter()
            .map(|&(valve, time)| {
                // Open the valve in the remaining time
                remaining_time.saturating_sub(time) as Pressure * info.flow_rate(valve)
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

    fn expand(&mut self, info: &GameInfo<N>, move_map: &MoveMap) -> Vec<GameState<N>> {
        // We can move to any closed valve and open it (if there is enough time)
        self.reachable_valves
            .iter()
            .map(|&(next_valve, time)| self.next_state(next_valve, time, info, move_map))
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
    move_map: MoveMap,
}

impl<const N: usize> PressureReleaseSearch<N> {
    fn new(info: GameInfo<N>, move_map: MoveMap) -> Self {
        Self { info, move_map }
    }

    fn search(&self) -> GameState<N> {
        // Do a modified A* search
        let mut open_set: BinaryHeap<GameState<N>> = BinaryHeap::new();
        open_set.push(GameState::<N>::start(&self.info, &self.move_map));

        while let Some(mut current) = open_set.pop() {
            if current.is_leaf() {
                return current;
            }

            open_set.extend(current.expand(&self.info, &self.move_map).into_iter());
        }

        panic!("Unexpected end of search without result!");
    }
}

pub struct Day16;

impl Day for Day16 {
    fn identifier(&self) -> &'static str {
        "16"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1::<59>(&input));
        println!("Part 2: {}", part_2(&input));
    }
}

fn part_1<const N: usize>(input: &str) -> Pressure {
    let info: GameInfo<N> = input.parse().unwrap();
    let move_map = info.compute_move_map();

    let pressure_search = PressureReleaseSearch::new(info, move_map);
    let result = pressure_search.search();
    result.score()
}

fn part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str =
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn should_parse_game_info() {
        let actual: GameInfo<10> = EXAMPLE_INPUT.parse().unwrap();
        let expected = GameInfo::<10> {
            adjacent_valves: AdjacentValves([
                vec![3, 8, 1],
                vec![2, 0],
                vec![3, 1],
                vec![2, 0, 4],
                vec![5, 3],
                vec![4, 6],
                vec![5, 7],
                vec![6],
                vec![0, 9],
                vec![8],
            ]),
            flow_rates: FlowRates([0, 13, 2, 20, 3, 0, 0, 22, 0, 21]),
            total_minutes: 30,
        };

        assert_eq!(actual, expected);
    }

    // #[test]
    // fn should_generate_starting_node() {
    //     let info: GameInfo<10> = EXAMPLE_INPUT.parse().unwrap();
    //     let actual = GameState::<10>::start(&info);
    //     let expected = GameState::<10> {
    //         cur_minute: 0,
    //         cur_pressure_release: 0,
    //         cur_valve: 0,
    //         open_valves: OpenValves::new(),
    //         reachable_valves: ReachableValves(vec![(1, 2), (2, 3), (3, 2), (4, 3), (7, 6), (9, 3)]),
    //         heuristic: 2154,
    //     };

    //     // We can't simply compare the state directly, as that only compares the score
    //     assert_eq!(actual.cur_minute, expected.cur_minute);
    //     assert_eq!(actual.cur_pressure_release, expected.cur_pressure_release);
    //     assert_eq!(actual.cur_valve, expected.cur_valve);
    //     assert_eq!(actual.open_valves, expected.open_valves);
    //     assert_eq!(actual.reachable_valves, expected.reachable_valves);
    //     assert_eq!(actual.heuristic, expected.heuristic);
    // }

    #[test]
    fn should_calculate_part_1_solution() {
        let actual = part_1::<10>(EXAMPLE_INPUT);

        assert_eq!(actual, 1651);
    }

    #[test]
    fn should_calculate_part_2_solution() {
        let actual = part_2(EXAMPLE_INPUT);

        assert_eq!(actual, 0);
    }
}
