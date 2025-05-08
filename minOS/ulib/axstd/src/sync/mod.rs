//! Useful synchronization primitives.

#[doc(no_inline)]
pub use core::sync::atomic;

#[cfg(feature = "alloc")]
#[doc(no_inline)]
pub use alloc::sync::{Arc, Weak};

#[doc(cfg(not(feature = "multitask")))]
pub use kspin::{SpinRaw as Mutex, SpinRawGuard as MutexGuard}; // never used in IRQ context
