extern crate core;

mod advent_runner;

mod year_2021;
mod year_2022;

use crate::advent_runner::AdventRunner;

fn main() {
    let advent_runner: AdventRunner = AdventRunner::new();

    advent_runner
        .set_year(2021)
        .run();
}