use crate::accounts::types::account::Account;
use anyhow::Error as AnyError;

pub trait Validator {
  fn validate_account(&self, account: &Account) -> Result<(), AnyError>;
}
