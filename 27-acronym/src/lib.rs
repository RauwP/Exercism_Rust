pub fn abbreviate(phrase: &str) -> String {
    let mut res = String::new();
    for word in phrase.split([' ', '-', '_']) {
        if !word.is_empty() {
            let mut chars = word.chars();
            let first = chars.next().unwrap();
            let mut prev = first;
            res.push(first.to_ascii_uppercase());
            for ch in chars {
                if ch.is_uppercase() && prev.is_lowercase() {
                    res.push(ch);
                }
                prev = ch;
            }
        }
    }
    res
}
