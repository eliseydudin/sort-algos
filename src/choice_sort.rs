fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut nums: Vec<u32> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    choice_sort(&mut nums);

    print_array(nums);
}

fn print_array<T: std::fmt::Display>(array: Vec<T>) {
    print!("[");
    for (i, elem) in array.iter().enumerate() {
        print!("{}", elem);

        if i != array.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
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
