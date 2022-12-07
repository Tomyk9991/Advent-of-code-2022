use std::fs;
use crate::advent_runner::Day;

fn part_1() {
    let input = fs::read_to_string("src/year_2021/day2/input.txt").unwrap();

    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;

    let _ = input.split("\n")
        .for_each(|line| {
            if let [command, unit_str] = line.split(" ").collect::<Vec<&str>>()[..] {
                let unit: i32 = unit_str.trim().parse().unwrap();

                match command {
                    "forward" => horizontal += unit,
                    "down" => vertical += unit,
                    "up" => vertical -= unit,
                    _ => { }
                }
            }
        });

    println!("Part one: X: {} Y: {} Resulting in: {}", horizontal, vertical, horizontal * vertical);
}

fn part_2() {
    let input = fs::read_to_string("src/year_2021/day2/input.txt").unwrap();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    let _ = input.split("\n")
        .for_each(|line| {
            if let [command, unit_str] = line.split(" ").collect::<Vec<&str>>()[..] {
                let unit: i32 = unit_str.trim().parse().unwrap();

                match command {
                    "forward" => {
                        horizontal += unit;
                        depth += (aim * unit)
                    },
                    "down" => aim += unit,
                    "up" => aim -= unit,
                    _ => { }
                }
            }
        });

    println!("Part one: X: {} Y: {} Resulting in: {}", horizontal, depth, horizontal * depth);
}

pub struct Day2;

impl Day for Day2 {
    fn date(&self) -> (i32, i32) {
        (2, 2021)
    }

    fn run(&self) {
        part_1();
        part_2();
    }
}

