// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
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

    println!("Enter the number to find its first occurrence:");
    let mut target_input = String::new();
    io::stdin().read_line(&mut target_input).expect("Failed to read line");
    let target: i32 = target_input.trim().parse().expect("Invalid input");

    if let Some(index) = find_first_occurrence(&arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("The number {} is not found in the array.", target);
    }
}
