use std::collections::HashMap;

// HEADER:
fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut word_counts = HashMap::new();

    for word in text.split_whitespace() {
        let word_lower = word.to_lowercase();
        *word_counts.entry(word_lower).or_insert(0) += 1;
    }
    word_counts
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_string() {
        let counts = word_frequency("");
        assert_eq!(counts.len(), 0);
    }
}

fn main() {
    println!("=== Word Counter ===");
    let test_word = "hello hello hello";
    let result = word_frequency(&test_word);
    println!("{:?}", result);
}
