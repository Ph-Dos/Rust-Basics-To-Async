pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    let iter = indices.iter();

    for i: usize in iter {
        slice[i] = value;
    }
}
