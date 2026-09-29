use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let target_map = build_frequency_map(word);
    for &candidate in possible_anagrams {
        if candidate.to_lowercase() == word.to_lowercase() || word.len() != candidate.len() {
            continue;
        }
        let candidate_map = build_frequency_map(candidate);
        if target_map == candidate_map {
            anagrams.insert(candidate);
        }
    }
    anagrams
}

fn build_frequency_map(word: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for c in word.to_lowercase().chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}
