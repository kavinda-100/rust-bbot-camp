#[cfg(test)]
mod test {
    use super::super::my_code::my_code::*;

    #[test]
    fn test_get_full_name() {
        let first_name = "John";
        let last_name = "Doe";
        let full_name = get_full_name(first_name, last_name);
        assert_eq!(full_name, format!("{} {}", first_name, last_name));
    }

    #[test]
    #[should_panic(expected = "First name or last name contains special characters")]
    fn test_get_full_name_special_chars() {
        let first_name = "*John";
        let last_name = "Doe";
        _ = get_full_name(first_name, last_name);
    }

    #[test]
    #[should_panic(expected = "First name and last name cannot be empty")]
    fn test_get_full_name_empty() {
        let first_name = "";
        let last_name = "Doe";
        _ = get_full_name(first_name, last_name);
    }
}