use std::cmp::Ordering;

pub fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    let (mut l, mut r) = (0, items.len());
    while l < r {
        let m = l + (r - l) / 2;
        match (&items[m]).cmp(target) {
            Ordering::Less => { l = m + 1 }
            Ordering::Greater => { r = m }
            Ordering::Equal => { return Some(m) }
        }
    }
    None
}