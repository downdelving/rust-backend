use crate::accounts::traits::password_hasher::PasswordHasher;
use anyhow::Error as AnyError;

pub struct ReversePasswordHasher {}

impl ReversePasswordHasher {
  pub fn new() -> Self {
    ReversePasswordHasher {}
  }
}

impl PasswordHasher for ReversePasswordHasher {
  fn hash_password(&self, password: &str) -> Result<String, AnyError> {
    Ok(password.to_string().chars().rev().collect())
  }
  fn check_password(&self, password: &str, hash: &str) -> Result<bool, AnyError> {
    Ok(password.to_string().chars().rev().collect::<String>() == hash)
  }
}

impl Default for ReversePasswordHasher {
  fn default() -> Self {
    ReversePasswordHasher::new()
  }
}

mod tests {
  use super::*;
  use crate::test::init as test_init;

  #[test]
  fn test_hash_password() {
    test_init();
    let hasher = ReversePasswordHasher::new();
    assert_eq!(hasher.hash_password("password").unwrap(), "drowssap");
  }

  #[test]
  fn test_check_password() {
    test_init();
    let hasher = ReversePasswordHasher::new();
    assert!(hasher.check_password("password", "drowssap").unwrap());
  }

  #[test]
  fn test_check_password_fail() {
    test_init();
    let hasher = ReversePasswordHasher::new();
    assert!(!hasher.check_password("password", "drowssap1").unwrap());
  }
}
