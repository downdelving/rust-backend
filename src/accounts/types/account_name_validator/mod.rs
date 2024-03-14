use crate::accounts::traits::validator::Validator;
use crate::accounts::types::account::Account;
use anyhow::anyhow;
use anyhow::Error as AnyError;
use regex::Regex;

pub struct AccountNameValidator {
  pub min_length: usize,
  pub max_length: usize,
  pub regex: Regex,
}

const DEFAULT_MIN_LENGTH: usize = 3;
const DEFAULT_MAX_LENGTH: usize = 20;
const DEFAULT_REGEX: &str = r"^[a-zA-Z0-9_]+$";

impl AccountNameValidator {
  pub fn new() -> AccountNameValidator {
    let min_length = DEFAULT_MIN_LENGTH;
    let max_length = DEFAULT_MAX_LENGTH;
    let regex = Regex::new(DEFAULT_REGEX).unwrap();
    AccountNameValidator {
      min_length,
      max_length,
      regex,
    }
  }
}

impl Default for AccountNameValidator {
  fn default() -> Self {
    Self::new()
  }
}

impl Validator for AccountNameValidator {
  fn validate_account(&self, account: &Account) -> Result<(), AnyError> {
    if account.username.len() < self.min_length {
      return Err(anyhow!("Username is too short"));
    }
    if account.username.len() > self.max_length {
      return Err(anyhow!("Username is too long"));
    }
    if !self.regex.is_match(&account.username) {
      return Err(anyhow!("Username contains invalid characters"));
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::accounts::types::account::Account;
  use crate::test::init as test_init;

  fn validate_account_test(username: &str, expected_ok: bool, expected_err: Option<&str>) {
    test_init();
    let validator = AccountNameValidator::new();
    let account = Account {
      id: "id".to_string(),
      username: username.to_string(),
      hashed_password: "password".to_string(),
      email: Some("email".to_string()),
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
    validate_account_test("usernamename", true, None);
    validate_account_test("username_name", true, None);
    validate_account_test("username name", false, Some("Username contains invalid characters"));
    validate_account_test("username`name", false, Some("Username contains invalid characters"));
    validate_account_test("username?name", false, Some("Username contains invalid characters"));
    validate_account_test("username'name", false, Some("Username contains invalid characters"));
    validate_account_test("us", false, Some("Username is too short"));
    validate_account_test("usernameusernameusername", false, Some("Username is too long"));
  }
}
