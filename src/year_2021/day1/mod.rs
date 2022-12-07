use std::fs;
use crate::advent_runner::Day;

fn part_2() {
    let input = fs::read_to_string("src/year_2021/day1/input.txt").unwrap();
    let vec = input.split("\n").collect::<Vec<&str>>();
    let mut increased_counter = 0;

    for mut i in 0..vec.len() - 3 {
        let first = {
            let fir = vec[i + 0].trim().parse::<i32>().unwrap();
            let sec = vec[i + 1].trim().parse::<i32>().unwrap();
            let thi = vec[i + 2].trim().parse::<i32>().unwrap();

            fir + sec + thi
        };

        let second = {
            let fir = vec[i + 1].trim().parse::<i32>().unwrap();
            let sec = vec[i + 2].trim().parse::<i32>().unwrap();
            let thi = vec[i + 3].trim().parse::<i32>().unwrap();

            fir + sec + thi
        };


        if second > first {
            increased_counter += 1;
        }
    }

    println!("Part two: {}", increased_counter);
}

fn part_1() {
    let input = fs::read_to_string("src/year_2021/day1/input.txt").unwrap();

    let vec = input.split("\n").collect::<Vec<&str>>();

    let mut increased_counter = 0;

    for i in 0..vec.len() - 1 {
        let curr = vec[i + 0].trim().parse::<i32>().unwrap();
        let next = vec[i + 1].trim().parse::<i32>().unwrap();

        if next > curr {
            increased_counter += 1;
        }
    }

    println!("Part one: {}", increased_counter);
}

pub struct Day1;

impl
Day for Day1 {
    fn date(&self) -> (i32, i32) {
        (1, 2021)
    }

    fn run(&self) {
        part_1();
        part_2();
    }
}