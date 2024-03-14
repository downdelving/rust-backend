use crate::accounts::traits::middleware::Middleware;
use crate::accounts::traits::storage::Storage;
use crate::accounts::types::account::Account;
use anyhow::Error as AnyError;

pub struct MediatedStorage<T>
where
  T: Storage,
{
  middleware: Box<dyn Middleware>,
  storage: T,
}

impl<T> MediatedStorage<T>
where
  T: Storage,
{
  pub fn new(middleware: Box<dyn Middleware>, storage: T) -> Self {
    Self { middleware, storage }
  }
}

impl<T> Storage for MediatedStorage<T>
where
  T: Storage,
{
  fn create_account(&mut self, account: Account) -> Result<(), AnyError> {
    self.middleware.handle(&account)?;
    self.storage.create_account(account)
  }

  fn get_account(&self, id: &str) -> Result<Account, AnyError> {
    self.storage.get_account(id)
  }

  fn update_account(&mut self, account: Account) -> Result<(), AnyError> {
    self.middleware.handle(&account)?;
    self.storage.update_account(account)
  }

  fn delete_account(&mut self, id: &str) -> Result<(), AnyError> {
    self.storage.delete_account(id)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::accounts::test_types::always_valid_validator::AlwaysValidValidator;
  use crate::accounts::test_types::never_valid_validator::NeverValidValidator;
  use crate::accounts::test_types::stubbed_storage::StubbedStorage;
  use crate::accounts::types::validator_middleware::ValidatorMiddleware;
  use crate::test::init as test_init;

  #[test]
  fn test_create_account1() {
    test_init();
    let always_valid = Box::new(ValidatorMiddleware::new(Box::new(AlwaysValidValidator)));
    let mut storage = MediatedStorage::new(always_valid, StubbedStorage::default());
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = storage.create_account(account);
    assert!(result.is_ok());
  }

  #[test]
  fn test_create_account2() {
    test_init();
    let never_valid = Box::new(ValidatorMiddleware::new(Box::new(NeverValidValidator)));
    let mut storage = MediatedStorage::new(never_valid, StubbedStorage::default());
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = storage.create_account(account);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "NeverValidValidator is never valid");
  }

  #[test]
  fn test_update_account1() {
    test_init();
    let always_valid = Box::new(ValidatorMiddleware::new(Box::new(AlwaysValidValidator)));
    let mut storage = MediatedStorage::new(always_valid, StubbedStorage::default());
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = storage.update_account(account);
    assert!(result.is_ok());
  }

  #[test]
  fn test_update_account2() {
    test_init();
    let never_valid = Box::new(ValidatorMiddleware::new(Box::new(NeverValidValidator)));
    let mut storage = MediatedStorage::new(never_valid, StubbedStorage::default());
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = storage.update_account(account);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "NeverValidValidator is never valid");
  }
}
