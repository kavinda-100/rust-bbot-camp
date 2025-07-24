#[cfg(test)]
mod test {
    use super::super::my_code::*;
    
    // Test for full name function with valid inputs
    #[test]
    fn test_get_full_name() {
        let first_name = "John";
        let last_name = "Doe";
        let full_name = get_full_name(first_name, last_name);
        assert_eq!(full_name, format!("{} {}", first_name, last_name));
    }
    // Test for full name function with special characters, expecting a panic
    #[test]
    #[should_panic(expected = "First name or last name contains special characters")]
    fn test_get_full_name_special_chars() {
        let first_name = "*John";
        let last_name = "Doe";
        _ = get_full_name(first_name, last_name);
    }
    // Test for full name function with empty first name, expecting a panic
    #[test]
    #[should_panic(expected = "First name and last name cannot be empty")]
    fn test_get_full_name_empty() {
        let first_name = "";
        let last_name = "Doe";
        _ = get_full_name(first_name, last_name);
    }
    // Test for addition function with valid inputs
    #[test]
    fn test_get_add_numbers() {
        let a = 5;
        let b = 10;
        let sum = get_add_numbers(a, b);
        assert_eq!(sum, a + b);
    }
    // Test for addition function with negative numbers, expecting a panic
    #[test]
    #[should_panic(expected = "Both numbers must be non-negative")]
    fn test_get_add_numbers_negative() {
        let a = -5;
        let b = 10;
        _ = get_add_numbers(a, b);
    }

    // Test for division function with valid inputs without panic using Result enum
    #[test]
    fn test_get_divide_numbers() {
        let a = 10;
        let b = 2;
        let result = get_divide_numbers(a, b);
        assert_eq!(result, Ok(a as f64 / b as f64));
    }

    // Test for division function with zero as divisor, expecting an error message using Result enum
    #[test]
    fn test_get_divide_numbers_zero() {
        let a = 10;
        let b = 0;
        let res = get_divide_numbers(a, b);
        assert_eq!(res, Err("Division by zero is not allowed".to_string()));
    }
}