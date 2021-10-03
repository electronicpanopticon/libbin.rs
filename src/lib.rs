pub fn boop() -> String {
    "Boop!".to_string()
}

pub fn greeting(name: String) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_test() {
        assert_eq!("Hello, World!".to_string(), greeting("World".to_string()));
    }
}
