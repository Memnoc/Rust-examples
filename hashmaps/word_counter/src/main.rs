use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    todo!()
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
}
