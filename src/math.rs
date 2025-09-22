/// Suma dos enteros
pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}

/// Resta dos enteros
pub fn resta(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma() {
        assert_eq!(suma(2, 2), 4);
        assert_eq!(suma(-1, 1), 0);
    }

    #[test]
    fn test_resta() {
        assert_eq!(resta(5, 3), 2);
        assert_eq!(resta(0, 3), -3);
    }
}
