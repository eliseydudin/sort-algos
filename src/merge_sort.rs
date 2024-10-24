fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut nums: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    merge_sort(&mut nums);

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

fn merge_sort(a: &mut Vec<usize>) {
    let mut b = a.clone();
    top_down_split_merge(a, 0, a.len(), &mut b);
}

fn top_down_merge(b: &mut Vec<usize>, begin: usize, middle: usize, end: usize, a: &mut Vec<usize>) {
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

fn top_down_split_merge(b: &mut Vec<usize>, begin: usize, end: usize, a: &mut Vec<usize>) {
    if end - begin <= 1 {
        return;
    }

    let middle = (end + begin) / 2;

    top_down_split_merge(a, begin, middle, b);
    top_down_split_merge(a, middle, end, b);
    top_down_merge(b, begin, middle, end, a);
}
