use std::time::Instant;
use std::array;

fn main() {
    let mut arr: [u32; 100] = array::from_fn(|i| i as u32 + 1);
    arr.reverse();
    let len = arr.len();

    let start = Instant::now();
    quicksort(&mut arr, 0, len - 1);
    let end = Instant::now();
    println!("{:?}", arr);

    println!(
        "Array of {} elements sorted by {:?}",
        arr.len(),
        end - start
    );
}

fn quicksort(arr: &mut [u32], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let pivot_idx = partition(arr, low, high);

    if pivot_idx != 0 {
        quicksort(arr, low, pivot_idx - 1);
    }
    quicksort(arr, pivot_idx + 1, high);
}

fn partition(arr: &mut [u32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut idx = low;

    for i in low..high {
        if arr[i] <= pivot {
            arr.swap(i, idx);
            idx += 1;
        }
    }

    arr.swap(high, idx);

    return idx;
}
