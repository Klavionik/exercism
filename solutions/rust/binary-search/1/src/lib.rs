pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut arr = array;
    let mut index = arr.len() / 2;

    loop {
        if arr.is_empty() {
            break;
        }

        let middle = arr.len() / 2;

        if arr[middle] == key {
            return Some(index);
        }

        let prev_len = arr.len();

        if key < arr[middle] {
            arr = &arr[..middle];
            index -= (prev_len - arr.len()) / 2
        } else {
            arr = &arr[middle + 1..];
            index += (prev_len - arr.len()) / 2
        }
    }

    None
}
