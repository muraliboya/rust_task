// Check if a number is prime in Rust
use std::io;

fn main() {
    println!("Enter a number to check if it's prime:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Please enter a valid number");

    let mut is_prime = true;

    if num <= 1 {
        is_prime = false;
    } else {
        for i in 2..=num / 2 {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
    }

    if is_prime {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}
