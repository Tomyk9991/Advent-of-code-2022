use std::fs;

pub fn run() {
    calculation(4);
    calculation(14);
}

fn calculation(unique_chars: usize) {
    let input = fs::read_to_string("src/day6/input.txt").unwrap();
    let mut lower_limit = 0;
    let mut upper_limit = unique_chars;

    while upper_limit < input.len() {
        let s = &input[lower_limit..upper_limit];
        if all_different_bitwise(s, unique_chars) {
            break;
        }

        lower_limit += 1;
        upper_limit += 1;
    }

    println!("{}", upper_limit);
}

fn all_different_bitwise(sequence: &str, check_amount: usize) -> bool {
    let mut data: u64 = 0b0;
    for i in 0..check_amount {
        let target = sequence.chars().nth(i).unwrap() as usize - 65;

        // is it set at this position?
        if (data & 1 << target) != 0 { // if there is a 1 already
            return false;
        }

        data |= (1 << target);
    }

    return true;
}