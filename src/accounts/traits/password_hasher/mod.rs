use anyhow::Error as AnyError;

pub trait PasswordHasher {
  fn hash_password(&self, password: &str) -> Result<String, AnyError>;
  fn check_password(&self, password: &str, hash: &str) -> Result<bool, AnyError>;
}
