use std::{
    fs::File,
    io::{Read, Write},
};

use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, Payload},
    Aes256GcmSiv, KeyInit,
};
use error::{AesError, Error};
use serde::Serialize;

mod error;
mod stream;

/// Serializes a data structure into a Vec<u8> using CBOR format.
///
/// # Arguments
///
/// * `data` - A reference to the data to be serialized.
///
/// # Returns
///
/// * `Result<Vec<u8>, Error>` - The serialized data as a vector of bytes, or an error if serialization fails.
pub fn serialize<T: Serialize>(data: &T) -> Result<Vec<u8>, Error> {
    let cbor_data = serde_cbor::to_vec(data)?;
    Ok(cbor_data)
}

/// Encrypts plaintext using AES-GCM-SIV algorithm.
///
/// # Arguments
///
/// * `key` - A reference to the 256-bit key for encryption.
/// * `nonce` - A reference to the 12-byte nonce.
/// * `plaintext` - A reference to the data to be encrypted.
/// * `associated_data` - A reference to the associated data.
///
/// # Returns
///
/// * `Result<Vec<u8>, Error>` - The encrypted data as a vector of bytes, or an error if encryption fails.
pub fn basic_encrypt(
    key: &[u8; 32],
    nonce: &[u8; 12],
    plaintext: &[u8],
    associated_data: &[u8],
) -> Result<Vec<u8>, Error> {
    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(key));
    let nonce = GenericArray::from_slice(nonce);
    let payload = Payload { msg: plaintext, aad: associated_data };
    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|_| Error::EcryptionError(AesError::Encrypt));

    ciphertext
}

/// Decrypts ciphertext using AES-GCM-SIV algorithm.
///
/// # Arguments
///
/// * `key` - A reference to the 256-bit key for decryption.
/// * `nonce` - A reference to the 12-byte nonce.
/// * `ciphertext` - A reference to the encrypted data.
/// * `associated_data` - A reference to the associated data.
///
/// # Returns
///
/// * `Result<Vec<u8>, Error>` - The decrypted data as a vector of bytes, or an error if decryption fails.
pub fn basic_decrypt(
    key: &[u8; 32],
    nonce: &[u8; 12],
    ciphertext: &[u8],
    associated_data: &[u8],
) -> Result<Vec<u8>, Error> {
    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(key));
    let nonce = GenericArray::from_slice(nonce);
    let payload = Payload { msg: ciphertext, aad: associated_data };
    let plaintext = cipher
        .decrypt(nonce, payload)
        .map_err(|_| Error::EcryptionError(AesError::Decrypt));

    plaintext
}

/// Generates a random 256-bit key.
///
/// # Returns
///
/// * `[u8; 32]` - The generated key.
pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    getrandom::getrandom(&mut key).expect("Error generating key");
    key
}

/// Saves a 256-bit key to a file.
///
/// # Arguments
///
/// * `key` - A reference to the 256-bit key to be saved.
/// * `file_path` - The path to the file where the key should be saved.
///
/// # Returns
///
/// * `std::io::Result<()>` - Returns `Ok(())` if successful, or an error if there was a problem saving the key.
pub fn save_key_to_file(key: &[u8; 32], file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(key)?;
    Ok(())
}

/// Loads a 256-bit key from a file.
///
/// # Arguments
///
/// * `file_path` - The path to the file from which the key should be loaded.
///
/// # Returns
///
/// * `std::io::Result<[u8; 32]>` - The loaded key or an error if there was a problem loading the key.
pub fn load_key_from_file(file_path: &str) -> std::io::Result<[u8; 32]> {
    let mut file = File::open(file_path)?;
    let mut key = [0u8; 32];
    file.read_exact(&mut key)?;
    Ok(key)
}
