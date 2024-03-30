// Implement a function that checks whether a given string is a palindrome or not.

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a string: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();

    if is_palindrome(input) {
        println!("'{}' is a palindrome", input);
    } else {
        println!("'{}' is not a palindrome", input);
    }
}

fn is_palindrome(input: &str) -> bool {
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < (chars.len() / 2) {
        if chars[i] != chars[chars.len() - 1 - i] {
            return false;
        }
        i += 1;
    }
    true
}
