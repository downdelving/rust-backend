use anyhow::anyhow;
use anyhow::Error as AnyError;

pub struct PasswordValidator {
  pub min_length: u8,
  pub max_length: u8,
  pub must_have_lowercase: bool,
  pub must_have_uppercase: bool,
  pub min_lowercase: u8,
  pub min_uppercase: u8,
  pub must_have_number: bool,
  pub must_have_special: bool,
  pub special_chars: Vec<char>,
}

const DEFAULT_SPECIAL_CHARS: [char; 32] = [
  '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[',
  '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];
const DEFAULT_MIN_LENGTH: u8 = 8;
const DEFAULT_MAX_LENGTH: u8 = 64;
const DEFAULT_MUST_HAVE_LOWERCASE: bool = true;
const DEFAULT_MUST_HAVE_UPPERCASE: bool = true;
const DEFAULT_MIN_LOWERCASE: u8 = 1;
const DEFAULT_MIN_UPPERCASE: u8 = 1;
const DEFAULT_MUST_HAVE_NUMBER: bool = true;
const DEFAULT_MUST_HAVE_SPECIAL: bool = true;

impl PasswordValidator {
  pub fn new() -> PasswordValidator {
    PasswordValidator {
      min_length: DEFAULT_MIN_LENGTH,
      max_length: DEFAULT_MAX_LENGTH,
      must_have_lowercase: DEFAULT_MUST_HAVE_LOWERCASE,
      must_have_uppercase: DEFAULT_MUST_HAVE_UPPERCASE,
      min_lowercase: DEFAULT_MIN_LOWERCASE,
      min_uppercase: DEFAULT_MIN_UPPERCASE,
      must_have_number: DEFAULT_MUST_HAVE_NUMBER,
      must_have_special: DEFAULT_MUST_HAVE_SPECIAL,
      special_chars: DEFAULT_SPECIAL_CHARS.to_vec(),
    }
  }

  pub fn validate_password(&self, password: &str) -> Result<(), AnyError> {
    if password.len() < self.min_length as usize {
      return Err(anyhow!("Password must be at least {} characters long", self.min_length));
    }
    if password.len() > self.max_length as usize {
      return Err(anyhow!("Password must be at most {} characters long", self.max_length));
    }
    if self.must_have_lowercase && password.chars().filter(|c| c.is_lowercase()).count() < self.min_lowercase as usize {
      return Err(anyhow!(
        "Password must have at least {} lowercase character(s)",
        self.min_lowercase
      ));
    }
    if self.must_have_uppercase && password.chars().filter(|c| c.is_uppercase()).count() < self.min_uppercase as usize {
      return Err(anyhow!(
        "Password must have at least {} uppercase character(s)",
        self.min_uppercase
      ));
    }
    if self.must_have_number && password.chars().filter(|c| c.is_numeric()).count() < 1 {
      return Err(anyhow!("Password must have at least 1 number(s)"));
    }
    if self.must_have_special && password.chars().filter(|c| self.special_chars.contains(c)).count() < 1 {
      return Err(anyhow!("Password must have at least 1 special character(s)."));
    }
    Ok(())
  }
}

impl Default for PasswordValidator {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;

  fn validate_password_test(password: &str, expected_ok: bool, expected_err: Option<&str>) {
    test_init();
    let validator = PasswordValidator::new();
    let result = validator.validate_password(password);

    match expected_ok {
      true => assert!(result.is_ok()),
      false => {
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), expected_err.unwrap());
      },
    }
  }

  #[test]
  fn test_validate_password() {
    validate_password_test("pass", false, Some("Password must be at least 8 characters long"));
    validate_password_test(
      "password",
      false,
      Some("Password must have at least 1 uppercase character(s)"),
    );
    validate_password_test(
      "PASSWORD",
      false,
      Some("Password must have at least 1 lowercase character(s)"),
    );
    validate_password_test("Password", false, Some("Password must have at least 1 number(s)"));
    validate_password_test(
      "Password1",
      false,
      Some("Password must have at least 1 special character(s)."),
    );
    validate_password_test("Password1!", true, None);
  }
}
