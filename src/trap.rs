use crate::asm;

#[derive(Debug)]
#[repr(C, packed)]
struct TrapFrame {
    ra: u32,
    gp: u32,
    tp: u32,
    t0: u32,
    t1: u32,
    t2: u32,
    t3: u32,
    t4: u32,
    t5: u32,
    t6: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u32,
    s5: u32,
    s6: u32,
    s7: u32,
    s8: u32,
    s9: u32,
    s10: u32,
    s11: u32,
    sp: u32,
}

#[macro_export]
macro_rules! read_csr {
    ($csr:literal) => {{
        let mut val: u32;
        unsafe {
            asm!(concat!("csrr {}, ", $csr), out(reg) val);
        }
        val
    }};
}

#[macro_export]
macro_rules! write_csr {
    ($csr:literal, $val:expr) => {{
        asm!(concat!("csrw ", $csr, ", {}"), in(reg) $val);
    }};
}

pub fn trap_entry() {
    unsafe {
        asm!(
            // repr(align(4))
            ".balign 4",

            // レジスタたちの退避．スタックに突っ込む．
            "csrw sscratch, sp",
            "addi sp, sp, -4 * 31",
            "sw ra, 4 * 0(sp)",
            "sw gp, 4 * 1(sp)",
            "sw tp, 4 * 2(sp)",
            "sw t0, 4 * 3(sp)",
            "sw t1, 4 * 4(sp)",
            "sw t2, 4 * 5(sp)",
            "sw t3, 4 * 6(sp)",
            "sw t4, 4 * 7(sp)",
            "sw t5, 4 * 8(sp)",
            "sw t6, 4 * 9(sp)",
            "sw a0, 4 * 10(sp)",
            "sw a1, 4 * 11(sp)",
            "sw a2, 4 * 12(sp)",
            "sw a3, 4 * 13(sp)",
            "sw a4, 4 * 14(sp)",
            "sw a5, 4 * 15(sp)",
            "sw a6, 4 * 16(sp)",
            "sw a7, 4 * 17(sp)",
            "sw s0, 4 * 18(sp)",
            "sw s1, 4 * 19(sp)",
            "sw s2, 4 * 20(sp)",
            "sw s3, 4 * 21(sp)",
            "sw s4, 4 * 22(sp)",
            "sw s5, 4 * 23(sp)",
            "sw s6, 4 * 24(sp)",
            "sw s7, 4 * 25(sp)",
            "sw s8, 4 * 26(sp)",
            "sw s9, 4 * 27(sp)",
            "sw s10, 4 * 28(sp)",
            "sw s11, 4 * 29(sp)",
            "csrr a0, sscratch",
            "sw a0, 4 * 30(sp)",
            "mv a0, sp",

            // handle_trap の呼び出し．
            "call {handle_trap}",

            // スタックからレジスタ戻す．
            "lw ra, 4 * 0(sp)",
            "lw gp, 4 * 1(sp)",
            "lw tp, 4 * 2(sp)",
            "lw t0, 4 * 3(sp)",
            "lw t1, 4 * 4(sp)",
            "lw t2, 4 * 5(sp)",
            "lw t3, 4 * 6(sp)",
            "lw t4, 4 * 7(sp)",
            "lw t5, 4 * 8(sp)",
            "lw t6, 4 * 9(sp)",
            "lw a0, 4 * 10(sp)",
            "lw a1, 4 * 11(sp)",
            "lw a2, 4 * 12(sp)",
            "lw a3, 4 * 13(sp)",
            "lw a4, 4 * 14(sp)",
            "lw a5, 4 * 15(sp)",
            "lw a6, 4 * 16(sp)",
            "lw a7, 4 * 17(sp)",
            "lw s0, 4 * 18(sp)",
            "lw s1, 4 * 19(sp)",
            "lw s2, 4 * 20(sp)",
            "lw s3, 4 * 21(sp)",
            "lw s4, 4 * 22(sp)",
            "lw s5, 4 * 23(sp)",
            "lw s6, 4 * 24(sp)",
            "lw s7, 4 * 25(sp)",
            "lw s8, 4 * 26(sp)",
            "lw s9, 4 * 27(sp)",
            "lw s10, 4 * 28(sp)",
            "lw s11, 4 * 29(sp)",
            "lw sp, 4 * 30(sp)",
            "sret",

            handle_trap = sym handle_trap,
            options(noreturn)
        )
    }
}

// 例外ハンドラ．ここに例外発生時の処理を記述．
fn handle_trap(trap_frame: TrapFrame) {
    let scause: u32 = read_csr!("scause");
    let stval: u32 = read_csr!("stval");
    let user_pc: u32 = read_csr!("sepc");

    panic!("Unexpected Trap: scause={:#010x}, stval={:#010x}, sepc={:#010x}, frame={:#010x?}", scause, stval, user_pc, trap_frame);
}

