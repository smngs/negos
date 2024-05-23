#![no_std]
#![no_main]
#![feature(naked_functions)]

// SBI とのインターフェイス
mod io;
mod sbicall;

// fmt::println
use crate::io::_print;

// 動的メモリ確保 (malloc)
mod malloc;
extern crate alloc;
use crate::malloc::BumpPointerAlloc;
use core::cell::UnsafeCell;
use core::ptr;

// パニック
use core::panic::PanicInfo;

// インラインアセンブリ
use core::arch::asm;

mod trap;
use crate::trap::trap_entry;

mod proc;
use crate::proc::ProcessManager;

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
        // let addr_bss_start = &mut __bss as *mut u9;
        let addr_bss_start = ptr::addr_of_mut!(__bss);
        let addr_bss_end = &__bss_end as *const u8;
        let length = addr_bss_end as usize - addr_bss_start as usize;
        ptr::write_bytes(addr_bss_start, 0, length);

        // 例外ハンドラの設定
        write_csr!("stvec", trap_entry as usize);
    }

    let hello = "fmt";
    println!("Hello World from {}!", hello);

    unsafe {
        ProcessManager::create_process(proc_a as u32);
        ProcessManager::create_process(proc_b as u32);
    }
    proc_a();

    unreachable!();
}

fn proc_a() {
    let mut a = 0;
    loop {
        print!("A={}", a);
        unsafe {
            asm!("add s0, s0, 1");
        }
        for _ in 0..1000000 {
            unsafe {
                asm!("nop")
            }
        }
        a += 1;
        unsafe {
            ProcessManager::yield_process();
        }
    }
}

fn proc_b() {
    let mut b = 0;
    loop {
        print!("B={}", b);
        unsafe {
            asm!("add s1, s1, 1");
        }
        for _ in 0..1000000 {
            unsafe {
                asm!("nop")
            }
        }
        b += 1;
        unsafe {
            ProcessManager::yield_process();
        }
    }
}
