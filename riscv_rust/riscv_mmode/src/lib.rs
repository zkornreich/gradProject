#![no_std]
#![no_main]

use core::panic::PanicInfo;
use riscv::register::{pmpcfg0, pmpaddr0, mcause, mepc, mtval};
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // Configure PMP0 to block access to 0x8000_1000 - 0x8000_1FFF
    unsafe {
        let pmpaddr_val = (0x8000_1000 >> 2) | 0b11; // NAPOT encoding for 4KB region
        pmpaddr0::write(pmpaddr_val);
        pmpcfg0::write(0x09); // R=0, W=0, X=0, A=NAPOT
    }

    // This read should cause a PMP trap
    unsafe {
        let ptr = 0x8000_1000 as *const u64;
        let _val = core::ptr::read_volatile(ptr);
    }

    loop {}
}

// Trap handler — required by riscv-rt
#[no_mangle]
pub unsafe extern "C" fn ExceptionHandler(_tf: &riscv_rt::TrapFrame) {
    let cause = mcause::read();
    let addr = mtval::read();
    let pc = mepc::read();

    // Breakpoint for debugging — or blink LED/log if you have I/O
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

