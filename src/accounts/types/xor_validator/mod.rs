use crate::accounts::traits::validator::Validator;
use crate::accounts::types::account::Account;
use anyhow::anyhow;
use anyhow::Error as AnyError;

pub struct XorValidator {
  pub validators: Vec<Box<dyn Validator>>,
}

impl XorValidator {
  pub fn new(validators: Vec<Box<dyn Validator>>) -> XorValidator {
    XorValidator { validators }
  }
}

impl Validator for XorValidator {
  fn validate_account(&self, account: &Account) -> Result<(), AnyError> {
    let mut valid_count = 0;
    for validator in &self.validators {
      if validator.validate_account(account).is_ok() {
        valid_count += 1;
      }
    }
    if valid_count == 1 {
      Ok(())
    } else {
      Err(anyhow!("Exactly one validator must validate the account"))
    }
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
    let validator = XorValidator::new(vec![Box::new(AlwaysValidValidator), Box::new(NeverValidValidator)]);
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = validator.validate_account(&account);
    assert!(result.is_ok());
  }

  #[test]
  fn test_validate_account2() {
    test_init();
    let validator = XorValidator::new(vec![Box::new(NeverValidValidator), Box::new(NeverValidValidator)]);
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
      "Exactly one validator must validate the account"
    );
  }

  #[test]
  fn test_validate_account3() {
    test_init();
    let validator = XorValidator::new(vec![Box::new(AlwaysValidValidator), Box::new(AlwaysValidValidator)]);
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
      "Exactly one validator must validate the account"
    );
  }
}
