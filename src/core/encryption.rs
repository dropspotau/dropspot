use std::io::{Read, Write};

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{self, Aead, AeadCore, KeyInit, OsRng, stream::EncryptorBE32},
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

const CHUNK_SIZE: usize = 4096;

pub(crate) fn encrypt_file(
    mut reader: impl Read,
    mut writer: impl Write,
) -> Result<Encryption, EncryptionError> {
    // The encryption key can be generated randomly:
    let aead = Aes256Gcm::generate_key(OsRng);

    // Stream encryption requires a 7-byte nonce, rather than a regular 12-byte nonce for AES-256-GCM
    let cipher = Aes256Gcm::new(&aead);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let nonce = Nonce::from_slice(&nonce[0..7]);

    // BE32 means it uses a 32-bit Big Endian counter under the hood
    // let mut stream_encryptor = EncryptorBE32::<Vec<u8>>::new(&aead, &nonce);
    let mut stream_encryptor = EncryptorBE32::from_aead(cipher, nonce.into());

    let mut buffer = [0u8; CHUNK_SIZE];

    loop {
        let read_count = reader.read(&mut buffer).unwrap();

        if read_count == CHUNK_SIZE {
            // Not the last chunk
            let ciphertext = stream_encryptor.encrypt_next(buffer.as_slice()).unwrap();
            writer.write_all(&ciphertext).unwrap();
        } else {
            // Final chunk (can be smaller than CHUNK_SIZE, or even 0 bytes)
            // Using encrypt_last secures the file against truncation attacks.
            let slice = &buffer[..read_count];
            let ciphertext = stream_encryptor.encrypt_last(slice).unwrap();
            writer.write_all(&ciphertext).unwrap();
            break;
        }
    }

    println!("KEY: {aead:?}");
    println!("NONCE: {nonce:?}");

    Ok(Encryption {
        key: aead.to_vec(),
        nonce: nonce.to_vec(),
    })
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
    let Ok(cipher) = Aes256Gcm::new_from_slice(&encryption.key) else {
        return Err(DecryptionError::CipherInvalidLengthError);
    };
    let nonce = Nonce::from_slice(&encryption.nonce);

    if let Err(e) = cipher.decrypt(&nonce, contents) {
        eprintln!("FAILED TO DECRYPT: {e:?}");
        return Err(DecryptionError::CipherError(e));
    }

    let plaintext = cipher
        .decrypt(&nonce, contents.as_ref())
        .map_err(|e| DecryptionError::CipherError(e))?;

    Ok(plaintext)
}
