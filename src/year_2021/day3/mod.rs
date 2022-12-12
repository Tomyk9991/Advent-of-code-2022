use std::fs;
use chrono::RoundingError::TimestampExceedsLimit;
use crate::advent_runner::Day;

struct ColumnIter<I> where I: Iterator {
    iterators: Vec<I>
}

impl<I, T> Iterator for ColumnIter<I> where I: Iterator<Item=T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Vec<T>> = self.iterators.iter_mut().map(|iter| iter.next()).collect();
        output
    }
}

pub struct Day3;
impl Day for Day3 {
    fn date(&self) -> (i32, i32) { (3, 2021) }

    fn run(&self) {
        let input: Vec<Vec<char>> = fs::read_to_string("src/year_2021/day3/input.txt")
        // let input: Vec<Vec<char>> = fs::read_to_string("src/year_2021/day3/test.txt")
            .unwrap()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<_>();
        let iterators = input.clone().into_iter().map(|v| v.into_iter()).collect();
        let column_iter = ColumnIter { iterators };

        let gamma_bits: Vec<i32> = column_iter.map(|column| {
            let column_string: String = column.iter().collect();
            return if column_string.matches("1").count() < column_string.matches("0").count() {
                1
            } else {
                0
            };
        }).collect();

        let mut gamma: i32 = 0;
        let mut epsilon: i32 = 0;
        for (index, bit) in gamma_bits.iter().rev().enumerate() {
            if *bit == 1 {
                gamma |= 1 << index;
            } else {
                epsilon |= 1 << index;
            }
        }

        println!("Part one: {} * {} = {}", gamma, epsilon, gamma * epsilon);
        
        let mut candidates: Vec<usize> = (0..input.len()).collect::<Vec<usize>>();
        let mut current_column = 0;

        
        while candidates.len() != 1 {
            let iterators = input.clone().into_iter().map(|v| v.into_iter()).collect();
            let column_iter = ColumnIter { iterators };
            
            let winner: i32 = column_iter.skip(current_column).take(1).map(|column| {
                let mut c = String::from("");
                for candidate_idx in &candidates {
                    c.push(column[*candidate_idx]);
                }
                let column_string: String = c;
                return if column_string.matches("1").count() >= column_string.matches("0").count() {
                    1
                } else {
                    0
                };
            }).collect::<Vec<i32>>()[0];
            
            for i in (0..candidates.len()).rev() {
                let candidate_idx = candidates[i];
                let item = &input[candidate_idx];
                if item[current_column].to_string().parse::<i32>().unwrap() != winner {
                    candidates.remove(i);
                }
            }
            
            current_column += 1;
        }

        let mut oxygen = 0;
        for (index, bit) in input[candidates[0]].iter().rev().enumerate() {
            let bit: i32 = if *bit == '1' { 1 } else { 0 };
            if bit == 1 {
                oxygen |= 1 << index;
            }
        }

        let mut candidates: Vec<usize> = (0..input.len()).collect::<Vec<usize>>();
        let mut current_column = 0;


        while candidates.len() != 1 {
            let iterators = input.clone().into_iter().map(|v| v.into_iter()).collect();
            let column_iter = ColumnIter { iterators };

            let winner: i32 = column_iter.skip(current_column).take(1).map(|column| {
                let mut c = String::from("");
                for candidate_idx in &candidates {
                    c.push(column[*candidate_idx]);
                }
                let column_string: String = c;
                return if column_string.matches("1").count() < column_string.matches("0").count() {
                    1
                } else {
                    0
                };
            }).collect::<Vec<i32>>()[0];

            for i in (0..candidates.len()).rev() {
                let candidate_idx = candidates[i];
                let item = &input[candidate_idx];
                if item[current_column].to_string().parse::<i32>().unwrap() != winner {
                    candidates.remove(i);
                }
            }

            current_column += 1;
        }

        let mut o2 = 0;
        for (index, bit) in input[candidates[0]].iter().rev().enumerate() {
            let bit: i32 = if *bit == '1' { 1 } else { 0 };
            if bit == 1 {
                o2 |= 1 << index;
            }
        }
        
        // oxygen = 3871
        // co2 = 10
        println!("Part two: {} * {} = {}", oxygen, o2, oxygen * o2);
    }
}