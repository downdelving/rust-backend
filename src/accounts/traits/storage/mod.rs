use crate::accounts::types::account::Account;
use anyhow::Error as AnyError;

pub trait Storage {
  fn create_account(&mut self, account: Account) -> Result<(), AnyError>;
  fn get_account(&self, id: &str) -> Result<Account, AnyError>;
  fn update_account(&mut self, account: Account) -> Result<(), AnyError>;
  fn delete_account(&mut self, id: &str) -> Result<(), AnyError>;
}
