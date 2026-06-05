use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Failed to hash password")]
    HashingFailed,

    #[error("Invalid hash format")]
    InvalidHash,
}

#[derive(Error, Debug)]
pub enum PasswordValidationError {
    #[error("Passwords require a minimum length of eight characters")]
    TooShort,

    #[error("Password requires a letter")]
    NoLetter,

    #[error("Password requires a number")]
    NoNumber,

    #[error("Password requires a symbol")]
    NoSymbol,
}

pub fn validate_password(password: &str) -> (bool, Vec<PasswordValidationError>) {
    const MIN_LENGTH: usize = 8;
    let mut errors: Vec<PasswordValidationError> = vec![];

    if password.len() < MIN_LENGTH {
        errors.push(PasswordValidationError::TooShort);
    }

    let letter_regex = Regex::new(r"[A-Za-z]").expect("Password validation letter regex failed");

    if !letter_regex.is_match(password) {
        errors.push(PasswordValidationError::NoLetter);
    }

    let number_regex = Regex::new(r"[0-9]").expect("Password validation number regex failed");

    if !number_regex.is_match(password) {
        errors.push(PasswordValidationError::NoNumber);
    }

    let symbol_regex = Regex::new(r#"[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]"#)
        .expect("Password validation symbol regex failed");

    if !symbol_regex.is_match(password) {
        errors.push(PasswordValidationError::NoSymbol);
    }

    if !errors.is_empty() {
        return (false, errors);
    }

    (true, errors)
}

/// Hash a password using Argon2id
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2id with default parameters (recommended for most use cases)
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| PasswordError::HashingFailed)
}

/// Verify a password against a stored hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| PasswordError::InvalidHash)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
