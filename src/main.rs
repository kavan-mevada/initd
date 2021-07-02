#![feature(test)]
#![feature(llvm_asm)]


extern crate test;


mod linux;
mod platform;
mod service;

use std::{env::Args, ffi::OsStr, fs::{File, OpenOptions, read_dir}, io::Result, path::PathBuf, rc::Rc};
use libc::{getpid, rusage, sleep};

use crate::{linux::{RLIMIT_NOFILE, execve, fork, getbcap, getrlimit, setbcap, setrlimit, wait4}, service::{Service, ServiceManager}};

fn main() -> Result<()> {    

    println!("Hello, world!");

    let mut servic_ctx = ServiceManager::init();

    let dir = read_dir("services")?
        .filter_map(Result::ok);

    let path = PathBuf::from("./services").join("wpa_supplicant.service");
    if let Ok(serv) = Service::new(&path) {
        servic_ctx.run(&serv)?;
    }


    return Ok(());



    for entry in dir {
        let path = entry.path();
        if path.extension() == Some(OsStr::new("service")) {


            let pid = fork();
            match pid {
                Ok(0) => {
                    if let Ok(serv) = Service::new(&path) {
                        servic_ctx.run(&serv);
                    }
                },
                _ => {
                    let mut status = -1;
                    let ret = wait4(-1, &mut status);
                    //dbg!(__WEXITSTATUS!(status), ret?);
                }
            }


            //execve(&["/bin/sh", "-c", "env"], &[]);

            // let cpid = fork();
            // match cpid {
            //     0 => {
            //         servic_ctx.run(s);
            //     },
            //     _ => {
            //         println!("Hello from parent! {}", unsafe { getpid() });
            //     }
            // }
        }
    }


    // child_a = fork();

    // if (child_a == 0) {
    //     /* Child A code */
    // } else {
    //     child_b = fork();

    //     if (child_b == 0) {
    //         /* Child B code */
    //     } else {
    //         /* Parent Code */
    //     }
    // }




    //dbg!(services);

    // let s = Service::from("myapp.service");
    // dbg!(s);

    let resource = RLIMIT_NOFILE;
    dbg!(getrlimit(resource));

    let resource = RLIMIT_NOFILE;
    dbg!(setrlimit(resource, 8192, 524288));

    setbcap(0x1fffbfbffff);

    let p = getbcap()?;
    println!("{:#016x}", p);


    // execve(
    //     &["/bin/sh", "-c", "env"],
    //     &["HOME=/", "PATH=/bin:/usr/bin", "TZ=UTC0"],
    // )?;


    

    execve(
        &["/bin/sh", "-c", "./target/debug/test"],
        &["HOME=/", "PATH=/bin:/usr/bin", "TZ=UTC0"]
    );

    //dbg!(std::io::Error::last_os_error());

    Ok(())
}




