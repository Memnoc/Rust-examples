// Try to change x to 6 (this will fail - think about why)
// Create a new variable z that's a floating-point number
// Try to use string formatting to print multiple variables in one println! statement
fn main() {
    println!("== Hello, Variables ==");

    let x = 5; // cannot change this because is not mutable
    let mut y = 10;
    let z = 3.4; // floating point

    println!("Values: {x}, {y}, {z}")
}
