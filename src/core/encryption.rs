use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum EncryptionError {
    #[error("Cipher error: {0}")]
    CipherError(aes_gcm::Error),
}

pub(crate) struct Encryption {
    /// AES-256 GCM key
    pub key: Vec<u8>,
    /// AES-256 GCM nonce
    pub nonce: Vec<u8>,
}

pub(crate) fn encrypt_file(contents: &[u8]) -> Result<(Encryption, Vec<u8>), EncryptionError> {
    // The encryption key can be generated randomly:
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, contents)
        .map_err(|e| EncryptionError::CipherError(e))?;

    let plaintext = cipher
        .decrypt(&nonce, ciphertext.as_ref())
        .map_err(|e| EncryptionError::CipherError(e))?;
    assert_eq!(&plaintext, b"plaintext message");

    Ok((
        Encryption {
            key: key.to_vec(),
            nonce: nonce.to_vec(),
        },
        ciphertext,
    ))
}

#[derive(Error, Debug)]
pub(crate) enum DecryptionError {
    #[error("Cipher invalid length")]
    CipherInvalidLengthError,
    #[error("Cipher error: {0}")]
    CipherError(aes_gcm::Error),
}

pub(crate) fn decrypt_file(
    encryption: &Encryption,
    contents: &[u8],
) -> Result<Vec<u8>, DecryptionError> {
    // The encryption key can be generated randomly:
    let Ok(cipher) = Aes256Gcm::new_from_slice(&encryption.key) else {
        return Err(DecryptionError::CipherInvalidLengthError);
    };
    let nonce = Nonce::from_slice(&encryption.nonce);
    let plaintext = cipher
        .decrypt(&nonce, contents.as_ref())
        .map_err(|e| DecryptionError::CipherError(e))?;

    Ok(plaintext)
}
