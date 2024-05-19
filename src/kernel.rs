#![no_std]
#![no_main]

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
use alloc::vec::Vec;
use alloc::string::String;
use core::ptr;

// パニック
use core::panic::PanicInfo;

// インラインアセンブリ
use core::arch::asm;

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
        let addr_bss_start = &mut __bss as *mut u8;
        let addr_bss_end = &__bss_end as *const u8;
        let length = addr_bss_end as usize - addr_bss_start as usize;
        ptr::write_bytes(addr_bss_start, 0, length);
    }

    let hello = "fmt";
    println!("Hello World from {}!", hello);

    let mut sum = 0;
    let mut test_vec = Vec::<usize>::new();
    for i in 0..=10 {
        test_vec.push(i);
        sum += i;
    }
    println!("test_vec = {:?}, sum = {}", test_vec, sum);

    let mut vec = Vec::<String>::new();
    loop {
        let line = read_line();
        vec.push(line);
        println!("{:?}", vec);
    }
    // panic!("panic panic panic minnaga awateteru");
}

