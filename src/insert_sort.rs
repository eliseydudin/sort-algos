use sort_algos::{print_array, read_u32_vec};

fn main() {
    let mut nums = read_u32_vec();
    insert_sort(&mut nums);
    print_array(nums);
}

fn insert_sort(nums: &mut Vec<u32>) {
    let mut i = 1;

    while i < nums.len() {
        let mut j = i;

        while j > 0 && nums[j - 1] > nums[j] {
            nums.swap(j - 1, j);
            j -= 1;
        }

        i += 1;
    }
}
