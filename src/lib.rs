pub fn greet(your_name: &str) -> String {
    format!("Hello: {your_name}!")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greet() {
        assert_eq!("Hello: aki!".to_string(), greet("aki"));
    }
}
