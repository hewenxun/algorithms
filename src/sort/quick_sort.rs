use rand::Rng;
use std::cmp::Ordering;

pub fn quick_sort<T: Ord + Clone>(items: &mut [T]) {
    _quick_sort(items, 0, items.len())
}

fn _quick_sort<T: Ord + Clone>(items: &mut [T], l: usize, r: usize) {
    if r - l < 2 { return }

    let (i, j) = partition(items, l, r);
    _quick_sort(items, l, i);
    _quick_sort(items, j, r)
}

fn partition<T: Ord + Clone>(items: &mut [T], l: usize, r: usize) -> (usize, usize) {
    let pivot = items[rand::thread_rng().gen_range(l..r)].clone();

    let (mut i, mut j, mut x) = (l, r, l);
    while x < j {
        match (&items[x]).cmp(&pivot) {
            Ordering::Less => {
                items.swap(x, i);
                x += 1;
                i += 1
            }
            Ordering::Greater => {
                j -= 1;
                items.swap(x, j)
            }
            Ordering::Equal => { x += 1 }
        }
    }

    (i, j)
}