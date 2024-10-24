use sort_algos::{print_array, read_u32_vec};

fn main() {
    let mut nums = read_u32_vec();
    bubble_sort(&mut nums);
    print_array(nums);
}

fn bubble_sort(arr: &mut Vec<u32>) {
    let n = arr.len();
    for i in 0..n {
        let mut max_index = 0;
        for j in 1..n - i {
            if arr[j] > arr[max_index] {
                max_index = j;
            }
        }
        arr.swap(max_index, n - 1 - i);
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
