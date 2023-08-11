fn main() {
    let mut vec = vec![1, 2, 4, 3, 8, 6];

    bubble_sort(&mut vec);
    dbg!(vec);
}

fn bubble_sort(vec: &mut Vec<u32>) {
    for i in 0..vec.len() {
        for j in 0..vec.len() - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j+1);
            }
        }
    }
}
