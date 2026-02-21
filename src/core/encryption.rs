use std::io::{Read, Write};

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{
        AeadCore, KeyInit, OsRng,
        stream::{DecryptorBE32, EncryptorBE32},
    },
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

// Size of each chunk for encryption and decryption
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
            let ciphertext = stream_encryptor
                .encrypt_last(&buffer[..read_count])
                .unwrap();
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
    mut reader: impl Read,
    mut writer: impl Write,
) -> Result<Vec<u8>, DecryptionError> {
    let Ok(cipher) = Aes256Gcm::new_from_slice(&encryption.key) else {
        return Err(DecryptionError::CipherInvalidLengthError);
    };
    let nonce = Nonce::from_slice(&encryption.nonce);
    let mut stream_decryptor = DecryptorBE32::from_aead(cipher, nonce.into());

    let mut buffer = [0u8; CHUNK_SIZE + 16];
    loop {
        let read_count = reader.read(&mut buffer).unwrap();

        // The last 16 bytes of each block are the authentication tag
        let is_last = read_count < CHUNK_SIZE + 16;

        if is_last {
            // The final chunk will be smaller than CHUNK_SIZE + 16.
            // If the file was exactly a multiple of CHUNK_SIZE, this will read exactly 16 bytes (just the final tag).
            let result = stream_decryptor.decrypt_last(&buffer[..read_count]);

            if let Err(e) = result {
                println!("Error decrypting last: {e:?}");
                return Err(DecryptionError::CipherError(e));
            }

            let plaintext = result.unwrap();
            writer.write_all(&plaintext).unwrap();
            break;
        } else {
            let result = stream_decryptor.decrypt_next(buffer.as_ref());

            if let Err(e) = result {
                println!("Error decrypting next: {e:?}");
                return Err(DecryptionError::CipherError(e));
            }

            let plaintext = result.unwrap();
            writer.write_all(&plaintext).unwrap();
        }
    }

    Ok(Vec::new())
}
