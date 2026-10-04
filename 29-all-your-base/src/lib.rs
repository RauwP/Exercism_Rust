#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    if let Some(&invalid_dig) = number.iter().find(|&&n| n >= from_base) {
        return Err(Error::InvalidDigit(invalid_dig));
    }
    let mut base10_res = 0;
    let mut res_in_to_base = <Vec<u32>>::new();
    for (i, &n) in number.iter().enumerate() {
        base10_res += n * from_base.pow((number.len() - 1 - i) as u32);
    }
    if base10_res == 0 {
        return Ok(vec![0]);
    }
    while base10_res > 0 {
        res_in_to_base.push(base10_res % to_base);
        base10_res /= to_base;
    }
    res_in_to_base.reverse();
    Ok(res_in_to_base)
}
