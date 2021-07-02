use std::{iter::once, mem::{ManuallyDrop, size_of}, rc::Rc, str, u8};

use crate::__SYSCALL;

type SizeT = isize;

pub(crate) const FORK: SizeT = 57;
pub(crate) const WAIT4: SizeT = 61;
pub(crate) const EXECVE: SizeT = 59;
pub(crate) const GETRLIMIT: SizeT = 97;
pub(crate) const PRCTL: SizeT = 157;
pub(crate) const SETRLIMIT: SizeT = 160;

const PR_CAPBSET_READ: SizeT = 23;
const PR_CAPBSET_DROP: SizeT = 24;

pub const RLIMIT_RSS: SizeT = 5;
pub const RLIMIT_AS: SizeT = 9;
pub const RLIMIT_MEMLOCK: SizeT = 8;
pub const RLIMIT_NOFILE: SizeT = 7;
pub const RLIMIT_NPROC: SizeT = 6;


pub(crate) fn fork() -> std::io::Result<SizeT> {
    unsafe { __SYSCALL!(FORK) }
}


pub(crate) fn wait4(pid: isize, status: &mut SizeT) -> std::io::Result<SizeT> {
    unsafe { __SYSCALL!(WAIT4, -1, status, 0, 0) }
}


pub(crate) fn getbcap<'a>() -> std::io::Result<SizeT> {
    let mut bnd = 0;
    (0..(size_of::<SizeT>() * 8)).for_each(|i| {
        if let Ok(1) = unsafe { __SYSCALL!(PRCTL, PR_CAPBSET_READ, i) } {
            bnd = bnd | (1 << i);
        }
    });

    Ok(bnd)
}

pub(crate) fn setbcap<'a>(caps: SizeT) {
    (0..(size_of::<SizeT>() * 8)).for_each(|i| {
        if (caps >> i) & 0x01 == 0 {
            unsafe { __SYSCALL!(PRCTL, PR_CAPBSET_DROP, i) };
        }
    });
}

pub(crate) fn getrlimit(resource: SizeT) -> std::io::Result<(u64, u64)> {
    let mut limit = (0, 0);
    unsafe { __SYSCALL!(GETRLIMIT, resource, &mut limit as *mut _) }.map(|_| limit)
}

pub(crate) fn setrlimit(resource: SizeT, slimit: SizeT, hlimit: SizeT) -> std::io::Result<SizeT> {
    let limit = (slimit, hlimit);
    unsafe { __SYSCALL!(SETRLIMIT, resource, &limit as *const _) }
}

pub(crate) fn execve<'a>(argv: &'a [&'a str], envp: &'a [&'a str]) -> std::io::Result<SizeT> {
    let args = to_str_vec(argv);
    unsafe {
        __SYSCALL!(
            EXECVE,
            args[0] as SizeT,
            args.as_ptr() as SizeT,
            to_str_vec(envp).as_ptr() as SizeT
        )
    }
}


fn str_as_cstr(s: &str) -> *const u8 {
    ManuallyDrop::new(s.bytes().chain(once(0)).collect::<Rc<_>>()).as_ptr() as _
}

fn to_str_vec<'a>(arr: &[&'a str]) -> Rc<[*const u8]> {
    arr.iter()
        .map(|&s| str_as_cstr(s))
        .chain(once(0 as _))
        .collect::<Rc<_>>()
}
