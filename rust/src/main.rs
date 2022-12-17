#![feature(iter_array_chunks)]
#![feature(let_chains)]
#![feature(iter_intersperse)]
use std::{env, process::exit};

use day_01::Day01;
use utils::Day;

use crate::{
    day_03::Day03, day_04::Day04, day_05::Day05, day_06::Day06, day_07::Day07, day_08::Day08,
    day_09::Day09, day_10::Day10, day_11::Day11, day_16::Day16,
};

mod day_01;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_16;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let days: Vec<Box<dyn Day>> = vec![
        Box::new(Day01),
        Box::new(Day03),
        Box::new(Day04),
        Box::new(Day05),
        Box::new(Day06),
        Box::new(Day07),
        Box::new(Day08),
        Box::new(Day09),
        Box::new(Day10),
        Box::new(Day11),
        Box::new(Day16),
    ];

    let identifier = args
        .get(1)
        .map(|arg| arg.as_str())
        .unwrap_or_else(|| days.last().unwrap().identifier());

    let day = days
        .iter()
        .find(|day| day.identifier() == identifier)
        .unwrap_or_else(|| {
            println!("Unknown day identifier '{identifier}', try something like '01'.");
            exit(1)
        });

    println!("RUNNING DAY {}:\n", day.identifier());
    day.run();
}
