pub fn convert_date_to_binary(date: String) -> String {
    let date: Vec<&str> = date.split('-').collect();
    let year = date[0].parse::<i32>().unwrap();
    let month = date[1].parse::<i32>().unwrap();
    let day = date[2].parse::<i32>().unwrap();
    format!("{year:b}-{month:b}-{day:b}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_date_to_binary() {
        assert_eq!(
            convert_date_to_binary("2080-02-29".to_string()),
            "100000100000-10-11101"
        );
        assert_eq!(
            convert_date_to_binary("1900-01-01".to_string()),
            "11101101100-1-1"
        );
    }
}
