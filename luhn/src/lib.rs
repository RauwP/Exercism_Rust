/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code
        .chars()
        .filter(|&ch| !ch.is_whitespace())
        .any(|ch| !ch.is_ascii_digit())
    {
        return false;
    }
    let parsed_code: Vec<u32> = code
        .chars()
        .rev()
        .filter_map(|ch| ch.to_digit(10))
        .collect();

    if parsed_code.len() > 1 {
        parsed_code
            .iter()
            .enumerate()
            .map(|(i, &ch)| {
                match ch
                    * match i % 2 {
                        0 => 1,
                        _ => 2,
                    } {
                    n if n <= 9 => n,
                    n => n - 9,
                }
            })
            .sum::<u32>()
            % 10
            == 0
    } else {
        false
    }
}
