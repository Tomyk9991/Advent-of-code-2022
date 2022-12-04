use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/day1/input.txt")
    .unwrap();

    let mut lines: Vec<u32> = input
        .split("\n\n")
        .map(|line| {
            line.split("\n")
                .flat_map(|num| num.parse::<u32>())
                .sum()
        })
        .collect::<Vec<u32>>();

    lines.sort_by(|a, b| b.cmp(a));
    
    println!("{}", lines[0]);
    println!("{}", lines.iter().take(3).sum::<u32>());
}