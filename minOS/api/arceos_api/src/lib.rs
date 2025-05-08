//! Public APIs and types for [ArceOS] modules
//!
//! [ArceOS]: https://github.com/arceos-org/arceos

#![no_std]
#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]
#![allow(unused_imports)]

#[cfg(any(
    feature = "alloc",
    feature = "dummy-if-not-enabled"
))]
extern crate alloc;

#[macro_use]
mod macros;
mod imp;

pub use axerrno::{AxError, AxResult};

/// Platform-specific constants and parameters.
pub mod config {
    pub use axconfig::*;
}

/// System operations.
pub mod sys {
    define_api! {
        /// Shutdown the whole system and all CPUs.
        pub fn ax_terminate() -> !;
    }
}

/// Time-related operations.
pub mod time {
    define_api_type! {
        pub type AxTimeValue;
    }

    define_api! {
        /// Returns the time elapsed since system boot.
        pub fn ax_monotonic_time() -> AxTimeValue;
        /// Returns the time elapsed since epoch, also known as realtime.
        pub fn ax_wall_time() -> AxTimeValue;
    }
}

/// Memory management.
pub mod mem {
    use core::{alloc::Layout, ptr::NonNull};

    define_api! {
        @cfg "alloc";
        /// Allocates a continuous memory blocks with the given `layout` in
        /// the global allocator.
        ///
        /// Returns [`None`] if the allocation fails.
        ///
        /// # Safety
        ///
        /// This function is unsafe because it requires users to manually manage
        /// the buffer life cycle.
        pub unsafe fn ax_alloc(layout: Layout) -> Option<NonNull<u8>>;
        /// Deallocates the memory block at the given `ptr` pointer with the given
        /// `layout`, which should be allocated by [`ax_alloc`].
        ///
        /// # Safety
        ///
        /// This function is unsafe because it requires users to manually manage
        /// the buffer life cycle.
        pub unsafe fn ax_dealloc(ptr: NonNull<u8>, layout: Layout);
    }
}

/// Standard input and output.
pub mod stdio {
    use core::fmt;
    define_api! {
        /// Reads a byte from the console, or returns [`None`] if no input is available.
        pub fn ax_console_read_byte() -> Option<u8>;
        /// Writes a slice of bytes to the console, returns the number of bytes written.
        pub fn ax_console_write_bytes(buf: &[u8]) -> crate::AxResult<usize>;
        /// Writes a formatted string to the console.
        pub fn ax_console_write_fmt(args: fmt::Arguments) -> fmt::Result;
    }
}

/// Multi-threading management.
pub mod task {
    define_api! {
        /// Current task is going to sleep, it will be woken up at the given deadline.
        ///
        /// If the feature `multitask` is not enabled, it uses busy-wait instead
        pub fn ax_sleep_until(deadline: crate::time::AxTimeValue);

        /// Current task gives up the CPU time voluntarily, and switches to another
        /// ready task.
        ///
        /// If the feature `multitask` is not enabled, it does nothing.
        pub fn ax_yield_now();

        /// Exits the current task with the given exit code.
        pub fn ax_exit(exit_code: i32) -> !;
    }
}

/// Input/output operations.
pub mod io {
    define_api_type! {
        pub type AxPollState;
    }
}

/// Re-exports of ArceOS modules.
///
/// You should prefer to use other APIs rather than these modules. The modules
/// here should only be used if other APIs do not meet your requirements.
pub mod modules {
    pub use axconfig;
    pub use axhal;
    pub use axlog;
    pub use axruntime;

    #[cfg(feature = "alloc")]
    pub use axalloc;
    #[cfg(feature = "paging")]
    pub use axmm;
}
