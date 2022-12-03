#![feature(iter_array_chunks)]
use std::{env, process::exit};

use day_01::Day01;
use utils::Day;

use crate::day_03::Day03;

mod day_01;
mod day_03;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let days: Vec<Box<dyn Day>> = vec![Box::new(Day01), Box::new(Day03)];

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
