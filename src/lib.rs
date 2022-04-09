#![warn(clippy::pedantic)]

#[must_use]
pub fn boop() -> String {
  "Boop!".to_string()
}

#[must_use]
pub fn greeting(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn greeting_test() {
    assert_eq!("Hello, World!".to_string(), greeting("World"));
  }
}
