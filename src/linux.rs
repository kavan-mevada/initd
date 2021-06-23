use std::{io::Read, iter::once, mem::{ManuallyDrop, size_of}, rc::Rc, str, u8};

use crate::__SYSCALL;

pub(crate) const PRCTL: usize = 157;
pub(crate) const EXECVE: usize = 59;
pub(crate) const GETRLIMIT: usize = 97;
pub(crate) const SETRLIMIT: usize = 160;

const PR_CAPBSET_READ: usize = 23;
const PR_CAPBSET_DROP: usize = 24;

pub const RLIMIT_RSS: usize = 5;
pub const RLIMIT_AS: usize = 9;
pub const RLIMIT_MEMLOCK: usize = 8;
pub const RLIMIT_NOFILE: usize = 7;
pub const RLIMIT_NPROC: usize = 6;


pub(crate) fn getbcap<'a>() -> std::io::Result<usize> {
    let mut bnd = 0;
    (0..(size_of::<usize>() * 8)).for_each(|i| {
        if let Ok(1) = unsafe { __SYSCALL!(PRCTL, PR_CAPBSET_READ, i) } {
            bnd = bnd | (1 << i);
        }
    });

    Ok(bnd)
}

pub(crate) fn setbcap<'a>(caps: usize) -> std::io::Result<()> {
    let mut caps = caps;
    let mut index = 0;
    while caps != 0 {

        if caps & 0x01 == 0 {
            unsafe { __SYSCALL!(PRCTL, PR_CAPBSET_DROP, index) }?;
        }

        caps = caps >> 1;
        index += 1;
    }

    Ok(())
}

pub(crate) fn getrlimit(resource: usize) -> std::io::Result<(u64, u64)> {
    let mut limit = (0, 0);
    unsafe { __SYSCALL!(GETRLIMIT, resource, &mut limit as *mut _) }.map(|_| limit)
}

pub(crate) fn setrlimit(resource: usize, slimit: usize, hlimit: usize) -> std::io::Result<usize> {
    let limit = (slimit, hlimit);
    unsafe { __SYSCALL!(SETRLIMIT, resource, &limit as *const _) }
}

pub(crate) fn execve<'a>(argv: &'a [&'a str], envp: &'a [&'a str]) -> std::io::Result<usize> {
    let args = to_str_vec(argv);
    unsafe {
        __SYSCALL!(
            EXECVE,
            args[0] as usize,
            args.as_ptr() as usize,
            to_str_vec(envp).as_ptr() as usize
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
