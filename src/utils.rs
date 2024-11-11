pub fn fast_int_sqrt(x: usize) -> usize {
    if x == 0 {
        return 0;
    }
    let mut result = x;
    while result * result > x {
        result = (result + x / result) / 2;
    }
    result
}
