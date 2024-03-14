use crate::accounts::traits::storage::Storage;
use crate::accounts::types::account::Account;
use anyhow::Error as AnyError;

pub struct StubbedStorage;

impl StubbedStorage {
  pub fn new() -> Self {
    StubbedStorage
  }
}

impl Default for StubbedStorage {
  fn default() -> Self {
    StubbedStorage::new()
  }
}

impl Storage for StubbedStorage {
  fn create_account(&mut self, _account: Account) -> Result<(), AnyError> {
    Ok(())
  }
  fn get_account(&self, _id: &str) -> Result<Account, AnyError> {
    Ok(Account {
      id: "aaaaaaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaaaaaa".to_string(),
      username: "username".to_string(),
      hashed_password: "Password1!".to_string(),
      email: Some("email@domain.com".to_string()),
    })
  }
  fn update_account(&mut self, _account: Account) -> Result<(), AnyError> {
    Ok(())
  }
  fn delete_account(&mut self, _id: &str) -> Result<(), AnyError> {
    Ok(())
  }
}
