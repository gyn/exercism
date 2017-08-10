#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T>(a: &[T], b: &[T]) -> Comparison
where
    T: PartialEq,
{
    let (needle, haystack, result) = if a.len() < b.len() {
        (a, b, Comparison::Sublist)
    } else if a.len() > b.len() {
        (b, a, Comparison::Superlist)
    } else {
        (a, b, Comparison::Equal)
    };

    if needle.is_empty() || haystack.windows(needle.len()).any(|w| w == needle) {
        result
    } else {
        Comparison::Unequal
    }
}
