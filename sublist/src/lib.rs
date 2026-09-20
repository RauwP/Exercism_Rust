#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        Comparison::Equal
    } else if is_sublist(a, b) {
        Comparison::Sublist
    } else if is_sublist(b, a) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

pub fn is_sublist<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    small.is_empty() || big.windows(small.len()).any(|w| w == small)
}
