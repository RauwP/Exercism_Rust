pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len();

    num == num
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap().pow(len as u32))
        .sum()
}
