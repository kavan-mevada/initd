#[macro_export]
macro_rules! __SYSCALL {
    ($($li:literal($y:expr)),+) => {{
        let ret: SizeT;
        llvm_asm!("syscall" : "={rax}"(ret)
                   : $($li($y)),+
                   : "rcx", "r11", "memory"
                   : "volatile");

        match ret {
            ret if ret < 0 => Err(std::io::Error::from_raw_os_error(
                (!(ret as isize) + 1) as _,
            )),

            _ => Ok(ret)
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

#[macro_export]
macro_rules! __WEXITSTATUS {
    ($y:expr) => { (($y) & 0xff00) >> 8 }
}
