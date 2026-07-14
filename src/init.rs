use std::sync::atomic::{AtomicU8, Ordering};

use thiserror::Error;

const UNINITIALIZED: u8 = 0;
const INITIALIZING: u8 = 1;
const INITIALIZED: u8 = 2;

static INIT_STATE: AtomicU8 = AtomicU8::new(UNINITIALIZED);

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum SodiumInitError {
    #[error("initialization failed")]
    InitializationFailed,
}

pub fn sodium_init() -> Result<(), SodiumInitError> {
    match INIT_STATE.compare_exchange(
        UNINITIALIZED,
        INITIALIZING,
        Ordering::AcqRel,
        Ordering::Acquire,
    ) {
        Ok(_) => {
            INIT_STATE.store(INITIALIZED, Ordering::Release);
            Ok(())
        }
        Err(INITIALIZED) => Ok(()),
        Err(INITIALIZING) => Ok(()),
        Err(_) => Err(SodiumInitError::InitializationFailed),
    }
}
