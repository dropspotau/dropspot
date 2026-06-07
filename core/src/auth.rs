use regex::Regex;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Authentication {
    pub token: String,
}

pub fn get_auth_headers(auth: Option<&Authentication>) -> HeaderMap {
    let mut headers = HeaderMap::new();

    if let Some(ref auth) = auth {
        headers.insert(
            "Authorization",
            format!("Bearer {}", auth.token).parse().unwrap(),
        );
    }

    headers
}

#[derive(Serialize, Deserialize, Error, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PasswordValidation {
    pub ok: bool,
    pub errors: Vec<PasswordValidationError>,
}

// Checks if a given password contains at least eight characters, with at least one letter, number and symbol
//
// This should match the password validation in server/src/auth/password.rs
// <param> password The password
// <returns> A tuple with a first boolean being whether the password is valid or not, and a list of any errors
#[wasm_bindgen(js_name = validatePassword)]
pub fn validate_password(password: &str) -> PasswordValidation {
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

    let ok = errors.is_empty();
    PasswordValidation { ok, errors }
}
