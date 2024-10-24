pub fn print_array<T: std::fmt::Display>(array: Vec<T>) {
    print!("[");
    for (i, elem) in array.iter().enumerate() {
        print!("{}", elem);

        if i != array.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}

pub fn read_u32_vec() -> Vec<u32> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let nums: Vec<u32> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    nums
}
