#![feature(llvm_asm)]
#![recursion_limit = "1024"]

mod linux;
mod platform;
mod service;

use std::io::Result;

use crate::{linux::{RLIMIT_NOFILE, execve, getbcap, getrlimit, setbcap, setrlimit}, service::Service};

fn main() -> Result<()> {
    println!("Hello, world!");

    let service = Service {
        Name: "sshd",
        CapBnd: None,
    };

    dbg!(service);

    let resource = RLIMIT_NOFILE;
    dbg!(getrlimit(resource)?);

    let resource = RLIMIT_NOFILE;
    dbg!(setrlimit(resource, 8192, 524288)?);

    setbcap(0x1fffbfbffff)?;

    let p = getbcap().unwrap();
    println!("{:#016x}", p);


    // execve(
    //     &["/bin/sh", "-c", "env"],
    //     &["HOME=/", "PATH=/bin:/usr/bin", "TZ=UTC0"],
    // )?;


    execve(
        &["/bin/sh", "-c", "./target/debug/test"],
        &["HOME=/", "PATH=/bin:/usr/bin", "TZ=UTC0"]
    )?;

    //dbg!(std::io::Error::last_os_error());

    Ok(())
}
