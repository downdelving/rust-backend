use crate::accounts::traits::validator::Validator;
use crate::accounts::types::account::Account;
use anyhow::Error as AnyError;

pub struct AlwaysValidValidator;

impl Validator for AlwaysValidValidator {
  fn validate_account(&self, _account: &Account) -> Result<(), AnyError> {
    Ok(())
  }
}

mod tests {
  use super::*;
  use crate::accounts::types::account::Account;
  use crate::test::init as test_init;

  #[test]
  fn test_validate_account() {
    test_init();
    let validator = AlwaysValidValidator;
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    assert!(validator.validate_account(&account).is_ok());
  }
}
