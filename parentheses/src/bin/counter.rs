use std::env;

pub fn is_parentheses_correct(input: &str) -> bool {
    // 0: (), 1: [], 2: {}
    let mut counters: [i32; 3] = [0, 0, 0];

    for elem in input.chars() {
        match elem {
            '(' => counters[0] += 1,
            ')' => { if counters[0] == 0 { return false } counters[0] -= 1},
            '[' => counters[1] += 1,
            ']' => { if counters[1] == 0 { return false } counters[1] -= 1},
            '{' => counters[2] += 1,
            '}' => { if counters[2] == 0 { return false } counters[2] -= 1},
            _ => panic!("ERROR! Non accepted character found {elem}"),
        }
    }
    counters[0] == 0 && counters[1] == 1 && counters[2] == 0

}

fn main() {
    let arg: String = match env::args().nth(1) {
        Some(val) => val,
        None => panic!("ERROR! No argument given to process!"),
    };

    println!("The string of parentheses is {}.", if is_parentheses_correct(&arg) { "valid" } else { "invalid" });
}