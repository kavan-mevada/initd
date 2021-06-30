#![feature(test)]
#![feature(llvm_asm)]


extern crate test;


mod linux;
mod platform;
mod service;

use std::{ffi::OsStr, fs::read_dir, io::Result, rc::Rc};
use crate::{linux::{RLIMIT_NOFILE, execve, getbcap, getrlimit, setbcap, setrlimit}, service::{Node, Service}};

fn main() -> Result<()> {
    println!("Hello, world!");



    let node = Node::NonEmpty(23, Rc::from(Node::NonEmpty(45, Rc::from(Node::Empty))));
    dbg!(&node);




    let dir = read_dir("services")?
        .filter_map(Result::ok);

    for entry in dir {
        let path = entry.path();
        if path.extension() == Some(OsStr::new("service")) {
            let s = Service::new(&path);
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