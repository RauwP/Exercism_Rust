pub fn reverse(input: &str) -> String {
    let mut rev = String::new();
    for c in input.chars() {
        rev.insert(0, c);
    }
    return rev;
    //alternetavily: retrurn input.chars().rev().collect();
}
