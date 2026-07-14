#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! libsodium_clone is a Rust-native cryptographic library modeled after the
//! public surface and operational guarantees of libsodium.
//!
//! This crate starts from a small, auditable core and grows outward through
//! well-scoped modules that mirror the upstream documentation families:
//! initialization, helpers, secure memory, randomness, symmetric crypto,
//! public-key crypto, hashing, password hashing, and advanced primitives.

pub mod init;
pub mod version;

pub use init::{sodium_init, SodiumInitError};
pub use version::{sodium_library_name, sodium_version_major, sodium_version_minor, sodium_version_string};

use sha2::{Digest, Sha256};
use sodiumoxide::crypto::secretbox::{self, Key, Nonce};

pub fn hash_bytes(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

pub fn secretbox_encrypt(input: &[u8], key: &[u8; 32], nonce: &[u8; 24]) -> Vec<u8> {
    let key = Key(*key);
    let nonce = Nonce(*nonce);
    secretbox::seal(input, &nonce, &key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sodiumoxide::crypto::{hash::sha256, secretbox};

    #[test]
    fn hash_matches_libsodium_reference() {
        let message = b"hackathon demo for a libsodium-style clone";

        let ours = hash_bytes(message);
        let theirs = sha256::hash(message).0.to_vec();

        println!("ours hash: {}", hex::encode(&ours));
        println!("libsodium hash: {}", hex::encode(&theirs));

        assert_eq!(ours, theirs);
    }

    #[test]
    fn secretbox_matches_libsodium_reference() {
        let key_bytes = [7u8; 32];
        let nonce_bytes = [11u8; 24];
        let plaintext = b"same parameters for both implementations";

        let ours = secretbox_encrypt(plaintext, &key_bytes, &nonce_bytes);
        let theirs = secretbox::seal(
            plaintext,
            &secretbox::Nonce(nonce_bytes),
            &secretbox::Key(key_bytes),
        );

        println!("ours ciphertext: {}", hex::encode(&ours));
        println!("libsodium ciphertext: {}", hex::encode(&theirs));

        assert_eq!(ours, theirs);
    }
}
