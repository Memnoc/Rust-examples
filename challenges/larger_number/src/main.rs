// Your task: Complete this function that:
// 1. Takes two numbers as parameters
// 2. Compares them using if/else
// 3. Returns the larger number
// Bonus: Handle the case where they're equal!

fn larger_number(num_one: i32, num_two: i32) -> i32 {
    if num_one > num_two {
        num_one
    } else {
        num_two
    }
}
fn main() {
    println!("== Larger number ==");
    let x = 5;
    let y = 10;

    let result = larger_number(x, y);
    println!("Larger number: {}", result);
}
