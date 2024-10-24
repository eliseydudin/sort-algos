# 26/09 Алгоритмы Сортировки   
   
1. Сортировка Пузырьком (aka. Bubble Sort). Меняются два подряд стоящих элемента, если они не в порядке. За каждый проход максимальный элемент ставит в конец.   
2. Сортировка Выбором (По максимуму, или минимуму). В массиве ищется максимум, и меняется местами с последним элементом массива, на следующем шаге также, только граница сдвигается на один справа.   
3. Сортировка Вставками. Каждый элемент массива (начиная со второго) вставляется в отсортированную часть массива.     
    ![Insertion-sort-example-300px.gif](files/insertion-sort-example-300px.gif)    
   
4. Сортировка Слиянием (aka. Merge Sort). Массив разделяется на отсортированные группы, после чего они обратно соединяются в один массив.   
  ![Merge-sort-example-300px.gif](files/merge-sort-example-300px.gif)    
    
# Имплементация   
Для функций, используемых всеми алгоритмами также создан файл `lib.rs`   
 
```rust
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

```
   
1. Bubble Sort    
```rust
use sort_algos::{print_array, read_u32_vec};

fn main() {
    let mut nums = read_u32_vec();
    bubble_sort(&mut nums);
    let mut clone = nums.clone();
    clone.sort();
    assert!(clone == nums);

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

```
   
2. Сортировка Выбором   
```rust
use sort_algos::{print_array, read_u32_vec};

fn main() {
    let mut nums = read_u32_vec();
    choice_sort(&mut nums);
    let mut clone = nums.clone();
    clone.sort();
    assert!(clone == nums);

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

```
3. Сортировка Вставками   
```rust
use sort_algos::{print_array, read_u32_vec};

fn main() {
    let mut nums = read_u32_vec();
    insert_sort(&mut nums);
    let mut clone = nums.clone();
    clone.sort();
    assert!(clone == nums);

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

```
4. Merge Sort    
```rust
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
```

# Запуск локально
Чтобы протестировать программы для себя, вам понадобятся:
```
python3+
cargo
```
Далее запустите нужный вам алгоритм используя `cargo r --bin algo_sort`.
Если вы хотите протестировать со своими тестами, то создайте директорию `tests` и поместите туда любой файл с тестами (они должны идти в одну строку) и запустите `python3 record_time.py test`