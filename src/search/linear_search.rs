pub fn linear_search<T: Eq>(items: &[T], target: &T) -> Option<usize> {
    for (index, item) in items.iter().enumerate() {
        if item.eq(target) {
            return Some(index)
        }
    }
    None
}