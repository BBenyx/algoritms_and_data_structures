mod dictionary;

fn main() {
    let mut my_dictionary = dictionary::Dictionary::new();
    my_dictionary.insert("O".to_string(), 8);
    my_dictionary.insert("J".to_string(), 2);
    my_dictionary.insert("Q".to_string(), 3);
    my_dictionary.insert("E".to_string(), 5);
    my_dictionary.insert("L".to_string(), 1);
    my_dictionary.insert("P".to_string(), 9);
    my_dictionary.insert("V".to_string(), 30);
    println!("{:?}", my_dictionary.pre_order());

    my_dictionary.remove("O".to_string());
    println!("{:?}", my_dictionary.pre_order());

    println!("{}\n\n\n", my_dictionary.read("E".to_string()).unwrap());


    let mut new_dictionary = dictionary::Dictionary::new();
    new_dictionary.insert("C", 3);
    println!("{}", new_dictionary.size);
    println!("{:?}", new_dictionary.read("C"));
    println!("{:?}", new_dictionary.read("Dog"));
    new_dictionary.insert("A", 13);
    
    println!("{:?}", new_dictionary.read("A"));
    new_dictionary.insert("B", 12);
    new_dictionary.insert("E", 12);
    new_dictionary.insert("D", 12);
    new_dictionary.insert("G", 12);
    new_dictionary.insert("F", 12);
    new_dictionary.insert("H", 12);
    new_dictionary.insert("I", 12);
    println!("{:?}", new_dictionary.remove("D"));
    println!("{:?}", new_dictionary.read("D"));
    println!("{:?}", new_dictionary.remove("Cow"));
    println!("{:?}", new_dictionary.pre_order());
}

