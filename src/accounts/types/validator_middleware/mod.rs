use crate::accounts::traits::middleware::Middleware;
use crate::accounts::traits::validator::Validator;
use crate::accounts::types::account::Account;
use anyhow::Error as AnyError;

pub struct ValidatorMiddleware {
  validator: Box<dyn Validator>,
}

impl ValidatorMiddleware {
  pub fn new(validator: Box<dyn Validator>) -> Self {
    Self { validator }
  }
}

impl Middleware for ValidatorMiddleware {
  fn handle(&self, account: &Account) -> Result<(), AnyError> {
    self.validator.validate_account(account)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::accounts::test_types::always_valid_validator::AlwaysValidValidator;
  use crate::accounts::test_types::never_valid_validator::NeverValidValidator;
  use crate::test::init as test_init;

  #[test]
  fn test_handle1() {
    test_init();
    let middleware = ValidatorMiddleware::new(Box::new(AlwaysValidValidator));
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = middleware.handle(&account);
    assert!(result.is_ok());
  }

  #[test]
  fn test_handle2() {
    test_init();
    let middleware = ValidatorMiddleware::new(Box::new(NeverValidValidator));
    let account = Account {
      id: "id".to_string(),
      username: "usernamename".to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
    };
    let result = middleware.handle(&account);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "NeverValidValidator is never valid");
  }
}
