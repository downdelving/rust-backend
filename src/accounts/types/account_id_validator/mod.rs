use crate::accounts::traits::validator::Validator;
use crate::accounts::types::account::Account;
use anyhow::anyhow;
use anyhow::Error as AnyError;
use regex::Regex;

pub struct AccountIdValidator {
  pub regex: Regex,
}

// Regex for verifying UUID format.
const DEFAULT_REGEX: &str = r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$";

impl AccountIdValidator {
  pub fn new() -> AccountIdValidator {
    let regex = Regex::new(DEFAULT_REGEX).unwrap();
    AccountIdValidator { regex }
  }
}

impl Default for AccountIdValidator {
  fn default() -> Self {
    Self::new()
  }
}

impl Validator for AccountIdValidator {
  fn validate_account(&self, account: &Account) -> Result<(), AnyError> {
    if account.id.len() != 36 {
      return Err(anyhow!("Account ID is not 36 characters long"));
    }
    if !self.regex.is_match(&account.id) {
      return Err(anyhow!("Account ID contains invalid characters"));
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::accounts::types::account::Account;
  use crate::test::init as test_init;

  fn validate_account_test(id: &str, expected_ok: bool, expected_err: Option<&str>) {
    test_init();
    let validator = AccountIdValidator::new();
    let account = Account {
      id: id.to_string(),
      username: "username".to_string(),
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
    validate_account_test("id", false, Some("Account ID is not 36 characters long"));
    validate_account_test("id-id-id-id-id", false, Some("Account ID is not 36 characters long"));
    validate_account_test("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa", true, None);
    validate_account_test(
      "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaaa",
      false,
      Some("Account ID is not 36 characters long"),
    );
    validate_account_test(
      "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaz",
      false,
      Some("Account ID contains invalid characters"),
    );
  }
}
