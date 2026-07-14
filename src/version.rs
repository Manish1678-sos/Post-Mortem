pub const LIBSODIUM_CLONE_NAME: &str = "libsodium_clone";
pub const LIBSODIUM_CLONE_VERSION_MAJOR: u32 = 0;
pub const LIBSODIUM_CLONE_VERSION_MINOR: u32 = 1;
pub const LIBSODIUM_CLONE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn sodium_library_name() -> &'static str {
    LIBSODIUM_CLONE_NAME
}

pub fn sodium_version_major() -> u32 {
    LIBSODIUM_CLONE_VERSION_MAJOR
}

pub fn sodium_version_minor() -> u32 {
    LIBSODIUM_CLONE_VERSION_MINOR
}

pub fn sodium_version_string() -> &'static str {
    LIBSODIUM_CLONE_VERSION
}
