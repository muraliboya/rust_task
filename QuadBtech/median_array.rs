// Given a sorted array of integers, implement a function that returns the median of the array.
use std::io;

fn find_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 0 {
        // If the length of the array is even
        let mid = n / 2;
        (arr[mid - 1] as f64 + arr[mid] as f64) / 2.0
    } else {
        // If the length of the array is odd
        arr[n / 2] as f64
    }
}

fn main() {
    println!("Enter a sorted array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let median = find_median(&arr);
    println!("The median of the array is: {}", median);
}
