mod counter;
mod couples;

use rand::RngExt;
use std::env;
use std::time::Instant;

type Tester = fn(&str) -> bool;

fn nested_rand_parentheses(size: usize) -> String {
    let mut return_string = String::new();

    for _ in 0..size {
        return_string.push(get_rand_parenthesis());
    }
    for i in (0..size).rev() {

        return_string.push(match return_string.chars().nth(i) {
            Some('(') => ')',
            Some('[') => ']',
            Some('{') => '}',
            _ => panic!("Invalid character found in func: nested_rand_parentheses!")
        } );
    }
    
    return_string
}

fn coupled_rand_prentheses(size: usize) -> String {
    let mut return_string = String::new();

    for _ in 0..size {
        let rand_char = get_rand_parenthesis();

        return_string.push(rand_char);
        return_string.push(match rand_char {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => panic!("Invalid character found in func: coupled_rand_parentheses!")
        })
    }

    return_string
}

fn get_rand_parenthesis() -> char {
    match rand::rng().random_range(0..3) {
        0 => return '(',
        1 => return '[',
        2 => return '{',
        _ => panic!("Out of range number generated!")
    }
}

fn nested_parentheses(n: usize) -> String {
    "(".repeat(n) + &")".repeat(n)
}

fn nested_curly_brackets(n: usize) -> String {
    "{".repeat(n) + &"}".repeat(n)
}



fn main() {

    let arg: String = match env::args().nth(2) {
        Some(val) => val,
        None => panic!("ERROR! No argument for number of parentheses!"),
    };

    let count = match arg.parse::<usize>() {
        Ok(val) => val,
        Err(_) => panic!("ERROR! {arg} is not a number!"),
    };

    let inputs = [
        ("Coupled random parenthesies", coupled_rand_prentheses(count)),
        ("Nested random parenthesies", nested_rand_parentheses(count)),
        ("Nested parentheses", nested_parentheses(count)),
        ("Nested curly brackets", nested_curly_brackets(count)),
    ];

    let testers: [(&str, Tester);2] = [
        ("Counter tester", counter::is_parentheses_correct),
        ("Couples tester", couples::is_parentheses_correct),
    ];

    for (input_name, input) in &inputs {

        println!("\nInput: {input_name}");
        for (tester_name, tester) in &testers {

            let start = Instant::now();
            let result = tester(input);
            let duration = start.elapsed();
            
            println!("  {}: {} --- {:?}",
                tester_name,
                if result { "[OK]" } else { "[NOK]" },
                duration
            );
        }
    }
    
}
