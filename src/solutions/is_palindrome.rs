pub fn is_palindrome1(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut n = x;
    let mut rev = 0;
    while n > 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    x == rev
}

pub fn is_palindrome2(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let s = x.to_string();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s.chars().nth(i).unwrap() != s.chars().nth(j).unwrap() {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

pub fn is_palindrome3(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let s = x.to_string();
    s.chars().eq(s.chars().rev())
    // s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome1(121), true);
        assert_eq!(is_palindrome1(-121), false);
        assert_eq!(is_palindrome1(10), false);
        assert_eq!(is_palindrome2(121), true);
        assert_eq!(is_palindrome2(-121), false);
        assert_eq!(is_palindrome2(10), false);
        assert_eq!(is_palindrome3(121), true);
        assert_eq!(is_palindrome3(-121), false);
        assert_eq!(is_palindrome3(10), false);
    }
}
