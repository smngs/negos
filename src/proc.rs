use crate::{print, println};
use crate::asm;
use core::ptr;

const PROCS_MAX: usize = 8;

pub struct ProcessManager {
    procs: [Process; PROCS_MAX],
    current_proc: &'static Process,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C, packed)]
pub struct Process {
    pid: i32,
    state: ProcessState,
    context: ProcessContext,
    stack: [u8; 4096]
}

impl Process {
    fn new() -> Process {
        Process {
            pid: 0,
            state: ProcessState::Unused,
            context: ProcessContext::new(),
            stack: [0x0; 4096]
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C, packed)]
pub struct ProcessContext {
    ra: u32,
    sp: u32,
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
}

impl ProcessContext {
    fn new() -> ProcessContext {
        ProcessContext { ra: 0, sp: 0, s0: 0, s1: 0, s2: 0, s3: 0, s4: 0, s5: 0, s6: 0, s7: 0, s8: 0, s9: 0, s10: 0, s11: 0 }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
enum ProcessState {
    Unused,
    Runnable,
    Running
}

const ARRAY_REPEAT_VALUE: Process = Process {
    pid: 0,
    state: ProcessState::Unused,
    context: ProcessContext { ra: 0, sp: 0, s0: 0, s1: 0, s2: 0, s3: 0, s4: 0, s5: 0, s6: 0, s7: 0, s8: 0, s9: 0, s10: 0, s11: 0 },
    stack: [0x0; 4096]
};

const IDLE_PROC: Process = Process {
    pid: -1,
    state: ProcessState::Unused,
    context: ProcessContext { ra: 0, sp: 0, s0: 0, s1: 0, s2: 0, s3: 0, s4: 0, s5: 0, s6: 0, s7: 0, s8: 0, s9: 0, s10: 0, s11: 0 },
    stack: [0x0; 4096]
};

static mut PROCS: [Process; PROCS_MAX] = [ARRAY_REPEAT_VALUE; PROCS_MAX];
static mut RUNNING_PROC: *mut Process = &mut IDLE_PROC as *mut Process; // 生ポインタ

impl ProcessManager {
    pub fn new() -> ProcessManager {
        ProcessManager {
            procs: [ARRAY_REPEAT_VALUE; PROCS_MAX],
            current_proc: &IDLE_PROC
        }
    }

    pub unsafe fn create_process(pc: u32) -> Option<()> {
        for (i, proc) in PROCS.as_mut().into_iter().enumerate() {
            let state = proc.state;
            if state == ProcessState::Unused {
                proc.state = ProcessState::Runnable;
                proc.pid = (i + 1) as i32;
                proc.context.ra = pc as u32;
                proc.context.sp = ptr::addr_of!(proc.stack[proc.stack.len() - 1]) as u32;
                return Some(());
            }
        }
        None
    }

    pub unsafe fn yield_process() {
        for proc in &mut PROCS.as_mut().into_iter() {
            let state = proc.state;
            if state == ProcessState::Runnable && proc.pid > 0 {
                if proc == &(*RUNNING_PROC) {
                    return
                }

                let prev_proc = RUNNING_PROC;
                RUNNING_PROC = proc;

                proc.state = ProcessState::Running;
                (*prev_proc).state = ProcessState::Runnable;

                println!("Prev: {:08x?}, Next: {:08x?}", (*prev_proc).context, proc.context);
                Self::switch_context(&mut (*prev_proc).context, &proc.context);
            }
        }
    }

    #[no_mangle]
    #[naked]
    pub extern "C" fn switch_context(prev: &mut ProcessContext, next: &ProcessContext) {
        unsafe {
            asm!(
                "sw ra, 0 * 4(a0)",
                "sw sp, 1 * 4(a0)",
                "sw s0, 2 * 4(a0)",
                "sw s1, 3 * 4(a0)",
                "sw s2, 4 * 4(a0)",
                "sw s3, 5 * 4(a0)",
                "sw s4, 6 * 4(a0)",
                "sw s5, 7 * 4(a0)",
                "sw s6, 8 * 4(a0)",
                "sw s7, 9 * 4(a0)",
                "sw s8, 10 * 4(a0)",
                "sw s9, 11 * 4(a0)",
                "sw s10, 12 * 4(a0)",
                "sw s11, 13 * 4(a0)",

                "lw ra, 0 * 4(a1)",
                "lw sp, 1 * 4(a1)",
                "lw s0, 2 * 4(a1)",
                "lw s1, 3 * 4(a1)",
                "lw s2, 4 * 4(a1)",
                "lw s3, 5 * 4(a1)",
                "lw s4, 6 * 4(a1)",
                "lw s5, 7 * 4(a1)",
                "lw s6, 8 * 4(a1)",
                "lw s7, 9 * 4(a1)",
                "lw s8, 10 * 4(a1)",
                "lw s9, 11 * 4(a1)",
                "lw s10, 12 * 4(a1)",
                "lw s11, 13 * 4(a1)",
                "ret",
                options(noreturn)
            )
        }
    }
}
