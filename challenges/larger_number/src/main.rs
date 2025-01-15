// Your task: Complete this function that:
// 1. Takes two numbers as parameters
// 2. Compares them using if/else
// 3. Returns the larger number
// Bonus: Handle the case where they're equal!

fn larger_number_match(num_one: i32, num_two: i32) -> i32 {
    match num_one.cmp(&num_two) {
        std::cmp::Ordering::Greater => num_one,
        std::cmp::Ordering::Less => num_two,
        std::cmp::Ordering::Equal => num_one,
    }
}
fn main() {
    println!("== Larger number ==");
    let x = 5;
    let y = 10;

    println!("Using match: {}", larger_number_match(x, y))
}
