#[cfg(target_os = "zkvm")]
use core::arch::asm;

use core::alloc::{GlobalAlloc, Layout};

extern crate alloc;

mod memory;
pub mod syscall;

pub const WORD_SIZE: usize = core::mem::size_of::<u32>();

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        const ZKVM_ENTRY: fn() = $path;

        mod zkvm_generated_main {
            #[no_mangle]
            fn main() {
                super::ZKVM_ENTRY()
            }
        }
    };
}

#[cfg(target_os = "zkvm")]
#[no_mangle]
unsafe extern "C" fn __start() {
    {
        extern "C" {
            fn main();
        }
        main()
    }
}

#[cfg(target_os = "zkvm")]
static STACK_TOP: u32 = 0x0020_0800; // TODO: put in whatever.

#[cfg(target_os = "zkvm")]
core::arch::global_asm!(
    r#"
.section .text._start;
.globl _start;
_start:
    .option push;
    .option norelax;
    la gp, __global_pointer$;
    .option pop;
    la sp, {0}
    lw sp, 0(sp)
    jal ra, __start;
"#,
    sym STACK_TOP
);

/// RUNTIME

struct SimpleAlloc;

unsafe impl GlobalAlloc for SimpleAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        memory::sys_alloc_aligned(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

#[global_allocator]
static HEAP: SimpleAlloc = SimpleAlloc;