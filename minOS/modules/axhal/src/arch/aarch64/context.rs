use core::arch::asm;
use memory_addr::VirtAddr;

/// Saved registers when a trap (exception) occurs.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TrapFrame {
    /// General-purpose registers (R0..R30).
    pub r: [u64; 31],
    /// User Stack Pointer (SP_EL0).
    pub usp: u64,
    /// Exception Link Register (ELR_EL1).
    pub elr: u64,
    /// Saved Process Status Register (SPSR_EL1).
    pub spsr: u64,
}

/// FP & SIMD registers.
#[repr(C, align(16))]
#[derive(Debug, Default)]
pub struct FpState {
    /// 128-bit SIMD & FP registers (V0..V31)
    pub regs: [u128; 32],
    /// Floating-point Control Register (FPCR)
    pub fpcr: u32,
    /// Floating-point Status Register (FPSR)
    pub fpsr: u32,
}

/// Saved hardware states of a task.
///
/// The context usually includes:
///
/// - Callee-saved registers
/// - Stack pointer register
/// - Thread pointer register (for thread-local storage, currently unsupported)
/// - FP/SIMD registers
///
/// On context switch, current task saves its context from CPU to memory,
/// and the next task restores its context from memory to CPU.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug)]
pub struct TaskContext {
    pub sp: u64,
    pub tpidr_el0: u64,
    pub r19: u64,
    pub r20: u64,
    pub r21: u64,
    pub r22: u64,
    pub r23: u64,
    pub r24: u64,
    pub r25: u64,
    pub r26: u64,
    pub r27: u64,
    pub r28: u64,
    pub r29: u64,
    pub lr: u64, // r30
}

impl TaskContext {
    /// Creates a new default context for a new task.
    pub const fn new() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }

    /// Initializes the context for a new task, with the given entry point and
    /// kernel stack.
    pub fn init(&mut self, entry: usize, kstack_top: VirtAddr, tls_area: VirtAddr) {
        self.sp = kstack_top.as_usize() as u64;
        self.lr = entry as u64;
        self.tpidr_el0 = tls_area.as_usize() as u64;
    }

    /// Switches to another task.
    ///
    /// It first saves the current task's context from CPU to this place, and then
    /// restores the next task's context from `next_ctx` to CPU.
    pub fn switch_to(&mut self, next_ctx: &Self) {
        unsafe { context_switch(self, next_ctx) }
    }
}

#[naked]
unsafe extern "C" fn context_switch(_current_task: &mut TaskContext, _next_task: &TaskContext) {
    asm!(
        "
        // save old context (callee-saved registers)
        stp     x29, x30, [x0, 12 * 8]
        stp     x27, x28, [x0, 10 * 8]
        stp     x25, x26, [x0, 8 * 8]
        stp     x23, x24, [x0, 6 * 8]
        stp     x21, x22, [x0, 4 * 8]
        stp     x19, x20, [x0, 2 * 8]
        mov     x19, sp
        mrs     x20, tpidr_el0
        stp     x19, x20, [x0]

        // restore new context
        ldp     x19, x20, [x1]
        mov     sp, x19
        msr     tpidr_el0, x20
        ldp     x19, x20, [x1, 2 * 8]
        ldp     x21, x22, [x1, 4 * 8]
        ldp     x23, x24, [x1, 6 * 8]
        ldp     x25, x26, [x1, 8 * 8]
        ldp     x27, x28, [x1, 10 * 8]
        ldp     x29, x30, [x1, 12 * 8]

        ret",
        options(noreturn),
    )
}
