// Implement a function that checks whether a given number is prime or not.
use std::io;

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please enter a valid number.");

    if is_prime(input) {
        println!("{} is a prime number.", input);
    } else {
        println!("{} is not a prime number.", input);
    }
}
