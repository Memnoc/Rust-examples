// Create a function that:
// 1. Takes a string
// 2. Reverses its words (not characters)
// 3. Returns the modified string
// Example: "hello world" -> "world hello"
// Bonus: Handle multiple spaces correctly

fn reverse_word(input: &str) -> String {
    let mut reversed_string = String::new();

    for c in input.chars().rev() {
        reversed_string.push(c);
    }
    reversed_string
}

fn main() {
    println!("== String Manipulation and Ownership ==");
    let text: &str = "hello";
    let reversed_string = reverse_word(text);

    println!("Reversed string: {}", reversed_string);
}
