fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut nums: Vec<u32> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    insert_sort(&mut nums);

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
