#![feature(test)]
#![feature(llvm_asm)]


extern crate test;


mod linux;
mod platform;
mod service;

use std::{ffi::OsStr, fs::{File, read_dir}, io::Result};

use crate::{linux::{RLIMIT_NOFILE, execve, getbcap, getrlimit, setbcap, setrlimit}, service::{Service}};

fn main() -> Result<()> {
    println!("Hello, world!");

    let dir = read_dir("services")?
        .filter_map(Result::ok);
    for files in dir {
        let path = files.path();

        if path.extension() == Some(OsStr::new("service")) {
            let s = Service::from(path);
            dbg!(s);
        }
    }

    //dbg!(services);

    // let s = Service::from("myapp.service");
    // dbg!(s);

    let resource = RLIMIT_NOFILE;
    dbg!(getrlimit(resource)?);

    let resource = RLIMIT_NOFILE;
    dbg!(setrlimit(resource, 8192, 524288)?);

    setbcap(0x1fffbfbffff);

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



#[bench]
fn rc_implementation(b: &mut test::Bencher) {
    b.iter(|| {
        let s = Service::from("myapp.service");
        assert!(s.Label == std::rc::Rc::from("system.sshd.org"), "Error reading!");
    })
}