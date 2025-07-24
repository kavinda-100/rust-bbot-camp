#[cfg(test)]
mod test {
    use super::super::my_code::my_code::*;

    #[test]
    fn test_get_full_name() {
        let first_name = "John";
        let last_name = "Doe";
        let full_name = get_full_name(first_name, last_name);
        assert_eq!(full_name, "John Doe");
    }
}