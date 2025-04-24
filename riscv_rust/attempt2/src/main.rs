#![no_std]
#![no_main]

use core::arch::asm;
use core::ptr;

const UART0_BASE: usize = 0x1000_0000;
const PROTECTED_BASE: usize = 0x8020_0000;

// UART Functions
fn uart_putc(c: u8) {
    unsafe {
        while ptr::read_volatile((UART0_BASE + 0x05) as *const u8) & 0x20 == 0 {}
        ptr::write_volatile((UART0_BASE + 0x00) as *mut u8, c);
    }
}

fn uart_puts(s: &str) {
    for b in s.bytes() {
        uart_putc(b);
    }
}

// Trap handler
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trap_handler() {
    uart_puts("TRAP: Memory Access Violation Caught!\n");
    loop {}
}

// PMP Setup
fn setup_pmp(addr: usize, size: usize) {
    let pmpaddr = (addr >> 2) | ((size / 2 - 1) >> 3);
    let cfg: u8 = 0x18; // NAPOT, no permissions

    unsafe {
        asm!(
            "csrw pmpaddr0, {0}",
            "csrw pmpcfg0, {1}",
            in(reg) pmpaddr,
            in(reg) cfg
        );
    }
}

// U-mode transition
fn enter_user_mode(user_fn: usize) -> ! {
    let user_stack: usize = 0x8100_0000;
    let mut mstatus: usize;

    unsafe {
        asm!("csrr {0}, mstatus", out(reg) mstatus);
        mstatus &= !0x1800; // Clear MPP (bits 11–12) to set to U-mode

        asm!(
            "csrw mepc, {0}",
            "csrw mstatus, {1}",
            "mv sp, {2}",
            "mret",
            in(reg) user_fn,
            in(reg) mstatus,
            in(reg) user_stack,
        );
    }

    loop {}
}

// User-mode code that violates PMP
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_code() {
    let val = unsafe{ptr::read_volatile(PROTECTED_BASE as *const u32)};
    let _ = val;
    uart_puts("User read succeeded! Should not happen!\n");
    loop {}
}

// Panic handler required by `#![no_std]`
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    uart_puts("PANIC!\n");
    loop {}
}

// Main entry point
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> ! {
    unsafe{ptr::write_volatile((UART0_BASE + 0x02) as *mut u8, 0x01)}; // Enable FCR

    uart_puts("Rust Hello World\n");
    uart_puts("Installing trap handler...\n");

    unsafe{asm!("csrw mtvec, {}", in(reg) trap_handler as usize)};

    uart_puts("Setting up PMP...\n");
    setup_pmp(PROTECTED_BASE, 16);

    uart_puts("Entering User Mode...\n");
    enter_user_mode(user_code as usize);
}
