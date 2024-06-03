use std::io;

fn main() {
    // Here’s a small programming problem: 
    // write a function that takes a string of words separated 
    // by spaces and returns the first word it finds in that string. 
    // If the function doesn’t find a space in the string, the whole string must be one word, 
    // so the entire string should be returned.
    println!("Input something: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    println!("Your Input Size: {}", input.len());

    let word = first_word(&input);
    println!("Got word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
