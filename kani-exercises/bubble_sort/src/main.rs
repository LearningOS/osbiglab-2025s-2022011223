fn real_bubble_sort(arr: &mut [u32], n: usize) {
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn wrong_bubble_sort(arr: &mut [u32], n: usize) {
    for i in 0..n {
        for j in i..n - i - 1 { // This is WRONG
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(6)]
fn check_bubble_sort() {
    const LIMIT: usize = 5;

    let mut arr: [u32; LIMIT] = kani::any();
    let length = kani::any();
    kani::assume(length <= LIMIT);

    // real_bubble_sort(&mut arr, length);
    wrong_bubble_sort(&mut arr, length);

    for i in 1..length {
        assert!(arr[i - 1] <= arr[i]);
    }
}

fn main() {
    // This is a placeholder main function.
    let mut a = [4294967295, 4294967295, 4294967294, 4294967295, 4294967295];
    let n = 3;
    wrong_bubble_sort(&mut a, n);
    println!("{:?}", a);
}
