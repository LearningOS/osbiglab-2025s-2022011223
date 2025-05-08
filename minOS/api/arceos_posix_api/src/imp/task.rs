use core::ffi::c_int;

/// Relinquish the CPU, and switches to another task.
///
/// For single-threaded configuration (`multitask` feature is disabled), we just
/// relax the CPU and wait for incoming interrupts.
pub fn sys_sched_yield() -> c_int {
    core::hint::spin_loop();
    0
}

/// Get current thread ID.
pub fn sys_getpid() -> c_int {
    syscall_body!(sys_getpid,
        Ok(2) // `main` task ID
    )
}

/// Exit current task
pub fn sys_exit(exit_code: c_int) -> ! {
    debug!("sys_exit <= {}", exit_code);
    axhal::misc::terminate();
}
