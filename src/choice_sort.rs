use sort_algos::{print_array, read_u32_vec};

fn main() {
    let mut nums = read_u32_vec();
    choice_sort(&mut nums);
    print_array(nums);
}

fn choice_sort(nums: &mut Vec<u32>) {
    let mut right_border = nums.len();

    while right_border != 0 {
        let mut max_ind = 0;
        for i in 0..right_border {
            if nums[i] > nums[max_ind] {
                max_ind = i;
            }
        }

        right_border -= 1;
        nums.swap(max_ind, right_border);
    }
}
