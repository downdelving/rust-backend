use crate::accounts::traits::validator::Validator;
use crate::accounts::types::account::Account;
use anyhow::anyhow;
use anyhow::Error as AnyError;

pub struct NeverValidValidator;

impl Validator for NeverValidValidator {
  fn validate_account(&self, _account: &Account) -> Result<(), AnyError> {
    Err(anyhow!("NeverValidValidator is never valid"))
  }
}

mod tests {
  use super::*;
  use crate::accounts::types::account::Account;
  use crate::test::init as test_init;

  #[test]
  fn test_validate_account() {
    test_init();
    let validator = NeverValidValidator;
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = validator.validate_account(&account);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "NeverValidValidator is never valid");
  }
}
