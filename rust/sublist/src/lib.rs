#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

#[inline]
fn contains<T>(short: &[T], longer: &[T]) -> Comparison
where
    T: PartialEq,
{
    let size = short.len();

    if longer.windows(size).any(|x| x == short) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}

pub fn sublist<T>(a: &[T], b: &[T]) -> Comparison
where
    T: PartialEq,
{
    if a.is_empty() && !b.is_empty() {
        return Comparison::Sublist;
    }

    if !a.is_empty() && b.is_empty() {
        return Comparison::Superlist;
    }

    if a.len() < b.len() {
        return contains(a, b);
    }

    if a.len() == b.len() && a == b {
        return Comparison::Equal;
    }

    if a.len() > b.len() && contains(b, a) == Comparison::Sublist {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}
