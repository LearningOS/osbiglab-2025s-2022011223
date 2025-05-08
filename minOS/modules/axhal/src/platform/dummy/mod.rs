#![allow(unused_variables)]
#![allow(dead_code)]

pub mod console {
    /// Writes a byte to the console.
    pub fn putchar(c: u8) {
        unimplemented!()
    }

    /// Reads a byte from the console, or returns [`None`] if no input is available.
    pub fn getchar() -> Option<u8> {
        unimplemented!()
    }
}

pub mod misc {
    /// Shutdown the whole system, including all CPUs.
    pub fn terminate() -> ! {
        unimplemented!()
    }
}

#[cfg(feature = "smp")]
pub mod mp {
    /// Starts the given secondary CPU with its boot stack.
    pub fn start_secondary_cpu(cpu_id: usize, stack_top: crate::mem::PhysAddr) {}
}

pub mod mem {
    /// Returns platform-specific memory regions.
    pub(crate) fn platform_regions() -> impl Iterator<Item = crate::mem::MemRegion> {
        core::iter::empty()
    }
}

pub mod time {
    /// Returns the current clock time in hardware ticks.
    pub fn current_ticks() -> u64 {
        0
    }

    /// Converts hardware ticks to nanoseconds.
    pub fn ticks_to_nanos(ticks: u64) -> u64 {
        ticks
    }

    /// Converts nanoseconds to hardware ticks.
    pub fn nanos_to_ticks(nanos: u64) -> u64 {
        nanos
    }

    /// Set a one-shot timer.
    ///
    /// A timer interrupt will be triggered at the specified monotonic time deadline (in nanoseconds).
    pub fn set_oneshot_timer(deadline_ns: u64) {}

    /// Return epoch offset in nanoseconds (wall time offset to monotonic clock start).
    pub fn epochoffset_nanos() -> u64 {
        0
    }
}

/// Initializes the platform devices for the primary CPU.
pub fn platform_init() {}

/// Initializes the platform devices for secondary CPUs.
#[cfg(feature = "smp")]
pub fn platform_init_secondary() {}
