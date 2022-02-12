use argon2::{Argon2, PasswordHash, PasswordVerifier};

pub fn check_password(password_hash: String, password_to_check: String) -> bool {
    let argon2 = Argon2::default();

    match PasswordHash::new(&password_hash) {
        Ok(parsed_hash) => argon2
            .verify_password(password_to_check.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}
