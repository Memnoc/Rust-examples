use std::collections::HashMap;

// INFO: Groups words that are anagram of each other
// Example: "listen" - "silent"
fn main() {
    println!("=== Anagram Grouper ===\n");
    let test_words = ["listen", "silent"];
    println!("{:?}", group_anagrams(&test_words));
}

fn group_anagrams<'a>(words: &'a [&'a str]) -> HashMap<String, Vec<&'a str>> {
    let mut anagram_groups: HashMap<String, Vec<&str>> = HashMap::new();

    for &word in words {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let sorted_key: String = chars.into_iter().collect();

        anagram_groups.entry(sorted_key).or_default().push(word);
    }

    anagram_groups
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_list() {
        let groups = group_anagrams(&[]);
        assert_eq!(groups.len(), 0);
    }

    #[test]
    fn test_no_anagram() {
        let words = ["one", "two", "three"];
        let groups = group_anagrams(&words);

        let mut found_group = None;
        groups.iter().for_each(|(_, group)| {
            if group.contains(&"eat") {
                found_group = Some(group);
            }
        });
    }

    #[test]
    fn test_anagram() {
        let words = ["listen", "silent", "eat", "tea", "ate", "nat", "tan"];
        let groups = group_anagrams(&words);

        let mut found_group = None;
        groups.iter().for_each(|(_, group)| {
            if group.contains(&"eat") {
                found_group = Some(group);
            }
        });
    }
}
