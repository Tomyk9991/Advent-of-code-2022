use std::fs;
use std::str::FromStr;
use crate::advent_runner::Day;

#[derive(Debug, Clone)]
struct Operation {
    operation: String
}

impl Operation {
    pub fn execute(&self, old: i32) -> i32 {
        return match self.operation.split("new = ").collect::<Vec<&str>>()[..][1].split(" ").collect::<Vec<&str>>()[..] {
            ["old", operation, "old"] => {
                let operation = operation.trim();
                let new = match operation {
                    "*" => old * old,
                    "+" => old + old,
                    _ => panic!("Unexpected operation")
                };

                new
            },
            ["old", operation, value] => {
                let operation = operation.trim();
                let value = value.trim().parse::<i32>().unwrap();
                let new = match operation {
                    "*" => old * value,
                    "+" => old + value,
                    _ => panic!("Unexpected operation")
                };

                new
            },
            [value, operation, "old"] => {
                let operation = operation.trim();
                let value = value.trim().parse::<i32>().unwrap();
                let new = match operation {
                    "*" => value * old,
                    "+" => value + old,
                    _ => panic!("Unexpected operation")
                };

                new
            },
            _ => panic!("Unexpected operation")
        };
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    starting_items: Vec<i32>,
    operation: Operation,
    test_divisible_by: i32,
    test_divisible_result: [usize; 2],
    inspections: usize
}

impl Monkey {
    pub fn increase_inspection(&mut self, amount: usize) {
        self.inspections += amount;
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(target: &str) -> Result<Self, Self::Err> {
        let mut starting_items = vec![];
        let mut operation = Operation { operation: String::from("") };
        let mut test_divisible_by: i32 = 0;
        let mut test_divisible_result: [usize; 2] = [0, 0];

        let lines = target.lines().collect::<Vec<&str>>();
        for (i, line) in lines.iter().enumerate() {
            let value: &str = line.split(":").collect::<Vec<&str>>()[1].trim();
            if i == 1 { // starting items
                let matching = &value.split(",").collect::<Vec<&str>>()[..];
                match matching {
                    [worry_level] => {
                        starting_items.push((*worry_level).trim().parse::<i32>().unwrap());
                    },
                    _ => {
                        for worry_level in matching {
                            starting_items.push((*worry_level).trim().parse::<i32>().unwrap());
                        }
                    }
                }
            } else if i == 2 { // operation
                operation.operation = value.trim().to_string();
            } else if i == 3 {
                match value.split(" ").collect::<Vec<&str>>()[..] {
                    ["divisible", "by", number] => {
                        test_divisible_by = number.trim().parse::<i32>().unwrap();
                    },
                    _ => panic!("Divisible by")
                }
            } else if i == 4 {
                match value.split(" ").collect::<Vec<&str>>()[..] {
                    ["throw", "to", "monkey", monkey_index] => {
                        test_divisible_result[0] = monkey_index.trim().parse::<usize>().unwrap();
                    },
                    _ => panic!("test divisible result 0")
                }
            } else if i == 5 {
                match value.split(" ").collect::<Vec<&str>>()[..] {
                    ["throw", "to", "monkey", monkey_index] => {
                        test_divisible_result[1] = monkey_index.trim().parse::<usize>().unwrap();
                    },
                    _ => panic!("test divisible result 1")
                }
            }
        }

        Ok(Monkey {
            starting_items,
            operation,
            test_divisible_by,
            test_divisible_result,
            inspections: 0
        })
    }
}

pub struct Day11;
impl Day for Day11 {
    fn date(&self) -> (i32, i32) { (11, 2022) }

    fn run(&self) {

        // let mut monkeys: Vec<Monkey> = fs::read_to_string("src/year_2022/day11/test.txt")
        //     .unwrap()
        let mut monkeys: Vec<Monkey> = fs::read_to_string("src/year_2022/day11/input.txt")
            .unwrap()
            .split("\r\n\r\n")
            .map(Monkey::from_str)
            .collect::<Result<_, _>>().expect("parsing");

        // 20 rounds
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let monkey = monkeys[i].clone();

                for starting_item in &monkey.starting_items {
                    let worry_level = (monkey.operation.execute(*starting_item) as f32 / 3.0) as i32;
                    let new_monkey_idx = if worry_level % monkey.test_divisible_by == 0 {
                        monkey.test_divisible_result[0]
                    } else {
                        monkey.test_divisible_result[1]
                    };

                    monkeys[i].increase_inspection(1);
                    monkeys[new_monkey_idx].starting_items.push(worry_level);
                }

                monkeys[i].starting_items.clear();
            }

            println!("{:#?}", monkeys);
        }

        monkeys.sort_by(|a, b| {
            b.inspections.cmp(&a.inspections)
        });

        println!("{:?}", monkeys[0].inspections * monkeys[1].inspections);
    }
}