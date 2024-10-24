fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut nums: Vec<u32> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    bubble_sort(&mut nums);

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
