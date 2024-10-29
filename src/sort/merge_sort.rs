use std::mem;

pub fn merge_sort<T: Ord + Clone + Default>(items: &mut [T]) {
    _merge_sort(items, 0, items.len())
}

fn _merge_sort<T: Ord + Clone + Default>(items: &mut [T], l: usize, r: usize) {
    if r - l < 2 { return }

    let m = l + (r - l) / 2;
    _merge_sort(items, l, m);
    _merge_sort(items, m, r);
    if items[m - 1] > items[m] { _merge(items, l, m, r) }
}

fn _merge<T: Ord + Clone + Default>(items: &mut [T], l: usize, m: usize, r: usize) {
    let mut items1: Vec<T> = items[l..m].to_vec();
    let mut items2: Vec<T> = items[m..r].to_vec();

    let (mut i1, mut i2) = (0, 0);
    while i1 < items1.len() && i2 < items2.len() {
        if items1[i1] <= items2[i2] {
            items[l + i1 + i2] = mem::take(&mut items1[i1]);
            i1 += 1;
        } else {
            items[l + i1 + i2] = mem::take(&mut items2[i2]);
            i2 += 1;
        }
    }

    while i1 < items1.len() {
        items[l + i1 + i2] = mem::take(&mut items1[i1]);
        i1 += 1;
    }
    while i2 < items2.len() {
        items[l + i1 + i2] = mem::take(&mut items2[i2]);
        i2 += 1;
    }
}