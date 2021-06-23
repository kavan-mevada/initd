#[macro_export]
macro_rules! __SYSCALL {
    ($($li:literal($y:expr)),+) => {{
        let ret: usize;
        llvm_asm!("syscall" : "={rax}"(ret)
                   : $($li($y)),+
                   : "rcx", "r11", "memory"
                   : "volatile");

        match ret {
            0 => Ok(ret),
            err => Err(std::io::Error::from_raw_os_error(
                (!(err as isize) + 1) as _,
            ))
        }
    }};

    ($n:expr) => { __SYSCALL!("{rax}"($n)) };
    ($n:expr, $a1:expr) => { __SYSCALL!("{rax}"($n), "{rdi}"($a1)) };
    ($n:expr, $a1:expr, $a2:expr) => { __SYSCALL!("{rax}"($n), "{rdi}"($a1), "{rsi}"($a2)) };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr) => { __SYSCALL!("{rax}"($n), "{rdi}"($a1), "{rsi}"($a2), "{rdx}"($a3)) };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => { __SYSCALL!("{rax}"($n), "{rdi}"($a1), "{rsi}"($a2), "{rdx}"($a3), "{r10}"($a4)) };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => { __SYSCALL!("{rax}"($n), "{rdi}"($a1), "{rsi}"($a2), "{rdx}"($a3), "{r10}"($a4), "{r8}"($a5)) };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => { __SYSCALL!("{rax}"($n), "{rdi}"($a1), "{rsi}"($a2), "{rdx}"($a3), "{r10}"($a4), "{r8}"($a5), "{r9}"($a6)) };
}
