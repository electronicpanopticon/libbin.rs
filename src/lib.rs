pub fn id() -> String {
    "World".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_test() {
        assert_eq!("World".to_string(), id());
    }
}
