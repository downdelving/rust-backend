use crate::accounts::traits::validator::Validator;
use crate::accounts::types::account::Account;
use anyhow::anyhow;
use anyhow::Error as AnyError;
use email_address::EmailAddress;

pub struct EmailValidator;

impl EmailValidator {
  pub fn new() -> Self {
    Self
  }
}

impl Default for EmailValidator {
  fn default() -> Self {
    EmailValidator::new()
  }
}

impl Validator for EmailValidator {
  fn validate_account(&self, account: &Account) -> Result<(), AnyError> {
    if let Some(email) = &account.email {
      if EmailAddress::is_valid(email) {
        Ok(())
      } else {
        Err(anyhow!("Invalid email"))
      }
    } else {
      Ok(())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::accounts::types::account::Account;
  use crate::test::init as test_init;

  fn validate_account_test(email: Option<&str>, expected_ok: bool, expected_err: Option<&str>) {
    test_init();
    let validator = EmailValidator::new();
    let account = Account {
      id: "id".to_string(),
      username: "username".to_string(),
      hashed_password: "password".to_string(),
      email: email.map(|s| s.to_string()),
    };
    let result = validator.validate_account(&account);

    match expected_ok {
      true => assert!(result.is_ok()),
      false => {
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), expected_err.unwrap());
      },
    }
  }

  #[test]
  fn test_validate_account() {
    test_init();
    validate_account_test(Some("email"), false, Some("Invalid email"));
    validate_account_test(Some(""), false, Some("Invalid email"));
    validate_account_test(Some("user@site.com"), true, None);
    validate_account_test(Some("user+tag@site.com"), true, None);
    validate_account_test(None, true, None);
  }
}
