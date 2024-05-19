use core::arch::asm;

#[allow(dead_code)]
pub struct SBIRet {
    pub error: i32,
    pub value: i32
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
pub fn sbi_call(mut a0: i32, mut a1: i32, mut a2: i32, mut a3: i32, mut a4: i32, mut a5: i32, mut fid: i32, mut eid: i32) -> SBIRet {
    unsafe {
        asm!(
            "ecall",
            inout("a0") a0,
            inout("a1") a1,
            out("a2") a2,
            out("a3") a3,
            out("a4") a4,
            out("a5") a5,
            out("a6") fid,
            out("a7") eid,
        );

        SBIRet {
            error: a0,
            value: a1 
        }
    }
}

