// Implement a function that returns the kth smallest element in a given array.
use std::io;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k <= 0 || k > arr.len() {
        return None;
    }
    
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    
    Some(sorted_arr[k - 1])
}

fn main() {
    println!("Enter the array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");
    let k: usize = k_input.trim().parse().expect("Invalid input");

    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is: {}", k, smallest),
        None => println!("Invalid input: k is out of range"),
    }
}
