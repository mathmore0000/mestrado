pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        let mut swapped = false;

        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

pub fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        let mut min_index = i;

        for j in i + 1..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        arr.swap(i, min_index);
    }
}

pub fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 1..n {
        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = key;
    }
}

pub fn merge_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut merged[..]);

    arr.copy_from_slice(&merged);
}

fn merge(left: &[i32], right: &[i32], merged: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged[k] = left[i];
            i += 1;
        } else {
            merged[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        merged[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        merged[k] = right[j];
        j += 1;
        k += 1;
    }
}

pub fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;

    arr.swap(pivot_index, len - 1);

    let pivot = arr[len - 1];
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}