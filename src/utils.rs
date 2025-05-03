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

#[cfg(test)]
mod tests {
    use super::fast_int_sqrt;

    #[test]
    fn test_fast_int_sqrt() {
        assert_eq!(fast_int_sqrt(4), 2);
        assert_eq!(fast_int_sqrt(5), 2);
        assert_eq!(fast_int_sqrt(32), 5);
    }
}
