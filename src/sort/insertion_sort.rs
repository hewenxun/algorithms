pub fn insertion_sort<T: Ord>(items: &mut [T]) {
    for i in 0..items.len() {
        let mut j = i;
        while j > 0 && items[j] < items[j - 1] {
            items.swap(j, j - 1);
            j -= 1;
        }
    }
}