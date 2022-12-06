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
        if all_different(s, unique_chars) {
            break;
        }

        lower_limit += 1;
        upper_limit += 1;
    }

    println!("{}", upper_limit);
}

fn all_different(sequence: &str, check_amount: usize) -> bool {
    for i in 0..check_amount {
        let target = sequence.chars().nth(i).unwrap();

        for j in 0..check_amount {
            if i != j && target == sequence.chars().nth(j).unwrap() {
                return false;
            }
        }
    }


    return true;
}