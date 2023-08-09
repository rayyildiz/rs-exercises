pub fn is_valid(card_no: String) -> Result<bool, String> {
    Err("not implemented".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_empty_string() {
        let result = is_valid(String::from(""));

        assert_eq!(false, result.is_ok());
        assert_eq!(true, result.is_err());
        assert_eq!("invalid card".to_string(), result.err().unwrap());
    }

    #[test]
    fn invalid_less_than_two_chars() {
        let result = is_valid(String::from("1"));

        assert_eq!(false, result.is_ok());
        assert_eq!(true, result.is_err());
        assert_eq!("invalid card".to_string(), result.err().unwrap());
    }

    #[test]
    fn invalid_card() {
        let result = is_valid(String::from("8273123273520569"));

        assert_eq!(false, result.is_ok());
        assert_eq!(true, result.is_err());
        assert_eq!("invalid card".to_string(), result.err().unwrap());
    }

    #[test]
    fn valid_card() {
        let result = is_valid(String::from("4539148803436467"));

        assert_eq!(true, result.is_ok());
        assert_eq!(false, result.is_err());
        assert_eq!(true, result.unwrap());
    }

    #[test]
    fn valid_card_with_spaces() {
        let result = is_valid(String::from("4539 1488 0343 6467"));

        assert_eq!(true, result.is_ok());
        assert_eq!(false, result.is_err());
        assert_eq!(true, result.unwrap());
    }

    #[test]
    fn invalid_chars() {
        let result = is_valid(String::from("123x"));

        assert_eq!(false, result.is_ok());
        assert_eq!(true, result.is_err());
        assert_eq!("invalid card".to_string(), result.err().unwrap());
    }
}
