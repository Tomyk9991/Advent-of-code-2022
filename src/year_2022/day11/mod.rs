use std::fs;
use crate::advent_runner::Day;

pub struct Day11;

impl Day for Day11 {
    fn date(&self) -> (i32, i32) { (11, 2022) }

    fn run(&self) {
        let input = fs::read_to_string("src/year_2022/day11/input.txt")
            .unwrap();
        
    }
}