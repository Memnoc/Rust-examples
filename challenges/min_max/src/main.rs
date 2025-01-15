// Create a function that:
// 1. Takes a vector of numbers
// 2. Returns both the minimum and maximum values
// 3. Handles the case of an empty vector (think about how to represent this!)
// Bonus: Make it generic to work with any comparable type

use std::collections;

fn find_min_max(collection: &Vec<i32>) -> Option<(i32, i32)> {
    if collection.is_empty() {
        return None; // handle empty vector case
    }

    let mut min = collection[0];
    let mut max = collection[0];

    for &num in collection.iter().skip(1) {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }

    Some((min, max))
}

fn main() {
    println!("== Min and Max");
    let numbers = vec![34, 25, 355];

    match find_min_max(&numbers) {
        Some((min, max)) => println!("Min: {}, Max: {}", min, max),
        None => println!("Vector is empty"),
    }
}
