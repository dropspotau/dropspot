// src/auth/token_service.rs
// JWT generation and validation

use chrono::Duration;
use dropspot_core::user::TokenPair;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use thiserror::Error;
use uuid::Uuid;

use crate::auth::claims::{AccessClaims, RefreshClaims};

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Token encoding failed")]
    EncodingFailed(#[from] jsonwebtoken::errors::Error),

    #[error("Token validation failed")]
    ValidationFailed,

    #[error("Token expired")]
    Expired,

    #[error("Invalid token type")]
    InvalidType,
}

/// Service for JWT operations
pub struct TokenService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_ttl: Duration,
    refresh_token_ttl: Duration,
}

impl TokenService {
    pub fn new(
        encoding_key: String,
        decoding_key: String,
        access_token_ttl_minutes: i64,
        refresh_token_ttl: i64,
    ) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(encoding_key.as_bytes()),
            decoding_key: DecodingKey::from_secret(decoding_key.as_bytes()),
            access_token_ttl: chrono::Duration::minutes(access_token_ttl_minutes),
            refresh_token_ttl: chrono::Duration::days(refresh_token_ttl),
        }
    }

    /// Generate access token
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        email: String,
    ) -> Result<String, TokenError> {
        let claims = AccessClaims::new(user_id, email, self.access_token_ttl);

        encode(&Header::default(), &claims, &self.encoding_key).map_err(TokenError::EncodingFailed)
    }

    /// Generate refresh token
    pub fn generate_refresh_token(&self, user_id: Uuid) -> Result<String, TokenError> {
        let claims = RefreshClaims::new(user_id, self.refresh_token_ttl);

        encode(&Header::default(), &claims, &self.encoding_key).map_err(TokenError::EncodingFailed)
    }

    /// Generate token pair (access + refresh)
    pub fn generate_token_pair(
        &self,
        user_id: Uuid,
        email: String,
    ) -> Result<TokenPair, TokenError> {
        Ok(TokenPair {
            access_token: self.generate_access_token(user_id, email)?,
            refresh_token: self.generate_refresh_token(user_id)?,
            token_type: "Bearer".to_string(),
            expires_in: self.access_token_ttl.num_seconds() as u64,
        })
    }

    /// Validate and decode access token
    pub fn validate_access_token(&self, token: &str) -> Result<AccessClaims, TokenError> {
        let mut validation = Validation::default();
        validation.validate_exp = true;

        let token_data: TokenData<AccessClaims> = decode(token, &self.decoding_key, &validation)
            .map_err(|_| TokenError::ValidationFailed)?;

        // Verify token type
        if token_data.claims.typ != "access" {
            return Err(TokenError::InvalidType);
        }

        Ok(token_data.claims)
    }

    /// Validate and decode refresh token
    pub fn validate_refresh_token(&self, token: &str) -> Result<RefreshClaims, TokenError> {
        let mut validation = Validation::default();
        validation.validate_exp = true;

        let token_data: TokenData<RefreshClaims> = decode(token, &self.decoding_key, &validation)
            .map_err(|_| TokenError::ValidationFailed)?;

        if token_data.claims.typ != "refresh" {
            return Err(TokenError::InvalidType);
        }

        Ok(token_data.claims)
    }
}
