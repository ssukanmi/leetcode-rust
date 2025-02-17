pub fn score_of_string(s: String) -> i32 {
    let cs = s.chars().map(|x| x as i32).collect::<Vec<i32>>();
    let mut score = 0;
    for i in 0..cs.len() - 1 {
        let a = cs[i];
        let b = cs[i+1];
        score += (a - b).abs();
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_of_string() {
        assert_eq!(score_of_string("hello".to_string()), 13);
        assert_eq!(score_of_string("zaz".to_string()), 50);
    }
}
