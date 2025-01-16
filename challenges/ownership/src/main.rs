// Create a function that:
// 1. Takes a string
// 2. Reverses its words (not characters)
// 3. Returns the modified string
// Example: "hello world" -> "world hello"
// Bonus: Handle multiple spaces correctly

fn reverse_word(input: &str) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();

    words.into_iter().rev().collect::<Vec<&str>>().join(" ")
}

fn main() {
    println!("== String Manipulation and Ownership ==");
    let text: &str = "hello world rust";
    let reversed = reverse_word(text);
    println!("Original: '{}'\nReversed: '{}'\n", text, reversed)
}
