#![no_std]
#![no_main]
#![feature(naked_functions)]

// SBI とのインターフェイス
mod io;
mod sbicall;

// fmt::println
use crate::io::{_print, read_line};

// 動的メモリ確保 (malloc)
mod malloc;
extern crate alloc;
use crate::malloc::BumpPointerAlloc;
use core::cell::UnsafeCell;
use core::hint::unreachable_unchecked;
use alloc::vec::Vec;
use alloc::string::String;
use core::ptr;

// パニック
use core::panic::PanicInfo;

// インラインアセンブリ
use core::arch::asm;

mod trap;
use crate::trap::trap_entry;

// ------------------------------------------------

#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc {
    head: UnsafeCell::new(0x8022_7000),
    end: 0x8422_7000,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

extern "C" {
    static __stack_top: u8;
    static mut __bss: u8;
    static __bss_end: u8;
}

#[no_mangle]
#[link_section = ".text.boot"]
pub extern "C" fn boot() -> ! {
    unsafe {
        asm!(
            "mv sp, {stack_top}",
            "j {kernel_main}",
            stack_top = in(reg) &__stack_top,
            kernel_main = sym kernel_main,
            options(noreturn)
        );
    }
}


#[no_mangle]
pub fn kernel_main () -> ! {
    unsafe {
        // BSS 領域のゼロクリア
        let addr_bss_start = &mut __bss as *mut u8;
        let addr_bss_end = &__bss_end as *const u8;
        let length = addr_bss_end as usize - addr_bss_start as usize;
        ptr::write_bytes(addr_bss_start, 0, length);

        // 例外ハンドラの設定
        write_csr!("stvec", trap_entry as usize);
    }

    let hello = "fmt";
    println!("Hello World from {}!", hello);

    unsafe {
        asm! ("unimp");
    }
    unreachable!();
}
