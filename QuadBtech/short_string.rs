// Given a string of words, implement a function that returns the shortest word in the string.
use std::io;

fn shortest_word(s: &str) -> &str {
    s.split_whitespace()
        .min_by_key(|word| word.len())
        .unwrap_or("")
}

fn main() {
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let shortest = shortest_word(input.trim());
    println!("The shortest word in the string is: {}", shortest);
}

