use crate::accounts::types::account::Account;
use anyhow::Error as AnyError;

pub trait Middleware {
  fn handle(&self, account: &Account) -> Result<(), AnyError>;
}
