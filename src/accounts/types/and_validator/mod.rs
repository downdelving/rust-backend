use crate::accounts::traits::validator::Validator;
use crate::accounts::types::account::Account;
use anyhow::anyhow;
use anyhow::Error as AnyError;

pub struct AndValidator {
  pub validators: Vec<Box<dyn Validator>>,
}

impl AndValidator {
  pub fn new(validators: Vec<Box<dyn Validator>>) -> AndValidator {
    AndValidator { validators }
  }
}

impl Validator for AndValidator {
  fn validate_account(&self, account: &Account) -> Result<(), AnyError> {
    for validator in &self.validators {
      let result = validator.validate_account(account);
      if result.is_err() {
        return Err(anyhow!(
          "A validator failed to validate the account: {}",
          result.err().unwrap()
        ));
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::accounts::test_types::always_valid_validator::AlwaysValidValidator;
  use crate::accounts::test_types::never_valid_validator::NeverValidValidator;
  use crate::accounts::types::account::Account;
  use crate::test::init as test_init;

  #[test]
  fn test_validate_account1() {
    test_init();
    let validator = AndValidator::new(vec![Box::new(AlwaysValidValidator), Box::new(NeverValidValidator)]);
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = validator.validate_account(&account);
    assert!(result.is_err());
    assert_eq!(
      result.err().unwrap().to_string(),
      "A validator failed to validate the account: NeverValidValidator is never valid"
    );
  }

  #[test]
  fn test_validate_account2() {
    test_init();
    let validator = AndValidator::new(vec![Box::new(NeverValidValidator), Box::new(NeverValidValidator)]);
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = validator.validate_account(&account);
    assert!(result.is_err());
    assert_eq!(
      result.err().unwrap().to_string(),
      "A validator failed to validate the account: NeverValidValidator is never valid"
    );
  }

  #[test]
  fn test_validate_account3() {
    test_init();
    let validator = AndValidator::new(vec![Box::new(AlwaysValidValidator), Box::new(AlwaysValidValidator)]);
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = validator.validate_account(&account);
    assert!(result.is_ok());
  }
}
