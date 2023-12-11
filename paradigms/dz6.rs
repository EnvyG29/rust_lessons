fn main() {
    let array: [i32; 8] = [1, 2, 3, 5, 7, 9, 10, 15];
    println!("{:?}", binary_find(2, &array));
}

fn binary_find(target: i32, arr: &[i32]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if target == arr[mid] {
            return Some(mid);
        } else if target < arr[mid] && mid > 0 {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    None
}
