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
