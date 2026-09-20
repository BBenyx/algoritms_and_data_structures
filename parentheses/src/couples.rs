//use std::env;

pub fn is_parentheses_correct(input: &str) -> bool {
    let mut copy= input.to_string();

    while !copy.is_empty() {

        match copy.find("()") {
            Some(pos) => copy.replace_range(pos..pos + 2, ""),
            None => match copy.find("[]") {

                Some(pos) => copy.replace_range(pos..pos + 2, ""),
                None => match copy.find("{}") {
                    
                    Some(pos) => copy.replace_range(pos..pos + 2, ""),
                    None => return false
                }
            }
        }
    }
    return true

}

/*fn main() {
    let arg: String = match env::args().nth(1) {
        Some(val) => val,
        None => panic!("ERROR! No argument given to process!"),
    };

    println!("\nThe string of parentheses is {}.\n", if is_parentheses_correct(&arg) { "valid" } else { "invalid" });
}*/