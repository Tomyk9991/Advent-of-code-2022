mod advent_runner;

mod year_2021;
mod year_2022;

use crate::advent_runner::{AdventRunner, Day};
use crate::year_2022::day6::Day6;

fn main() {
    let advent_runner: AdventRunner = AdventRunner::new();

    advent_runner
        .set_year(2022)
        .run();
}