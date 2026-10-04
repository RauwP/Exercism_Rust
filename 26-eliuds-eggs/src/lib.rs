pub fn egg_count(display_value: u32) -> usize {
    let mut num = display_value;
    let mut res = 0;
    while num > 0 {
        if num % 2 == 1 {
            res += 1;
        }
        num /= 2;
    }
    res
}
