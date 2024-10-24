use sort_algos::{print_array, read_u32_vec};

fn main() {
    let mut nums = read_u32_vec();
    merge_sort(&mut nums);
    let mut clone = nums.clone();
    clone.sort();
    assert!(clone == nums);

    print_array(nums);
}

fn merge_sort(a: &mut Vec<u32>) {
    let mut b = a.clone();
    top_down_split_merge(a, 0, a.len(), &mut b);
}

fn top_down_merge(b: &mut Vec<u32>, begin: usize, middle: usize, end: usize, a: &mut Vec<u32>) {
    let mut i = begin;
    let mut j = middle;

    for k in begin..end {
        if i < middle && (j >= end || a[i] <= a[j]) {
            b[k] = a[i];
            i += 1;
        } else {
            b[k] = a[j];
            j += 1;
        }
    }
}

fn top_down_split_merge(b: &mut Vec<u32>, begin: usize, end: usize, a: &mut Vec<u32>) {
    if end - begin <= 1 {
        return;
    }

    let middle = (end + begin) / 2;

    top_down_split_merge(a, begin, middle, b);
    top_down_split_merge(a, middle, end, b);
    top_down_merge(b, begin, middle, end, a);
}
