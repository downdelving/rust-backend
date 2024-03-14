pub struct Account {
  // UUID.
  pub id: String,
  // Username, must be unique.
  pub username: String,
  // Email, must be unique if not None.
  pub email: Option<String>,
  // Hashed password.
  pub hashed_password: String,
}
