use crate::accounts::traits::password_hasher::PasswordHasher;
use anyhow::Error as AnyError;
use bcrypt;

pub struct BcryptPasswordHasher {
  cost: u32,
}

impl BcryptPasswordHasher {
  pub fn new(cost: u32) -> Self {
    BcryptPasswordHasher { cost }
  }
}

impl PasswordHasher for BcryptPasswordHasher {
  fn hash_password(&self, password: &str) -> Result<String, AnyError> {
    bcrypt::hash(password, self.cost).map_err(|e| e.into())
  }
  fn check_password(&self, password: &str, hash: &str) -> Result<bool, AnyError> {
    bcrypt::verify(password, hash).map_err(|e| e.into())
  }
}
