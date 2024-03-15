use crate::accounts::traits::storage::Storage;
use crate::accounts::types::account::Account;
use anyhow::anyhow;
use anyhow::Error as AnyError;

pub struct InMemoryStorage {
  accounts: Vec<Account>,
}

impl InMemoryStorage {
  pub fn new() -> Self {
    InMemoryStorage { accounts: Vec::new() }
  }
}

impl Default for InMemoryStorage {
  fn default() -> Self {
    InMemoryStorage::new()
  }
}

impl Storage for InMemoryStorage {
  fn create_account(&mut self, account: Account) -> Result<(), AnyError> {
    if self.accounts.iter().any(|a| a.id == account.id) {
      return Err(anyhow!("Account already exists"));
    }
    self.accounts.push(account);
    Ok(())
  }

  fn get_account(&self, id: &str) -> Result<Account, AnyError> {
    self
      .accounts
      .iter()
      .find(|a| a.id == id)
      .cloned()
      .ok_or_else(|| anyhow!("Account not found"))
  }

  fn update_account(&mut self, account: Account) -> Result<(), AnyError> {
    let index = self
      .accounts
      .iter()
      .position(|a| a.id == account.id)
      .ok_or_else(|| anyhow!("Account not found"))?;
    self.accounts[index] = account;
    Ok(())
  }

  fn delete_account(&mut self, id: &str) -> Result<(), AnyError> {
    let index = self
      .accounts
      .iter()
      .position(|a| a.id == id)
      .ok_or_else(|| anyhow!("Account not found"))?;
    self.accounts.remove(index);
    Ok(())
  }
}
