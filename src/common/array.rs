pub fn to_2d_index(idx: usize, width: usize) -> (usize, usize) {
    let x = idx % width;
    let y = idx / width;
    (x, y)
}
