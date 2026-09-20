//use std::env;

pub fn is_parentheses_correct(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for elem in input.chars() {
        match elem {
            '(' | '[' | '{' => stack.push(elem),
            ')' | ']' | '}' => {
                match stack.pop() {
                    Some('(') if elem == ')' => {},
                    Some('[') if elem == ']' => {},
                    Some('{') if elem == '}' => {},
                    _ => return false
                }
            },
            _ => return false
        }
    }
    stack.len() == 0
    

}

/*fn main() {
    let arg: String = match env::args().nth(1) {
        Some(val) => val,
        None => panic!("ERROR! No argument given to process!"),
    };

    println!("\nThe string of parentheses is {}.\n", if is_parentheses_correct(&arg) { "valid" } else { "invalid" });
}*/