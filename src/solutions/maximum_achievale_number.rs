pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
    // nums + (1*t) = ans - (1*t)
    // nums + t = ans - t
    // nums + t + t = ans
    // ans = nums + 2t
    num + (t * 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_the_maximum_achievable_x() {
        assert_eq!(the_maximum_achievable_x(4, 1), 6);
        assert_eq!(the_maximum_achievable_x(3, 2), 7);
    }
}
