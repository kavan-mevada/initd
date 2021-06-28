use std::fs::File;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct pollfd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

extern "C" {
    fn poll(
        fds: *mut pollfd, 
        nfds: u64, 
        timeout: i32
    ) -> i32;
}


fn main() {
    let addr = Addr::kobject(1);
    let addr_r = Addr::route(/* RTMGRP_IPV4_IFADDR */ 0x10);
    let s = Socket::new(&addr, /* SOCK_RAW */ 3 );
    loop {
        let msg = s.recv(0x10);

        println!("msg: {:?}", msg);
    }
}


fn main()  {
    unsafe {
        let fds = &mut [std::mem::zeroed::<pollfd>(); 2];

        use std::os::unix::io::AsRawFd;
        let fd = File::open("test").unwrap().as_raw_fd();

            /* watch stdin for input */
            fds[0].fd = 1;
            fds[0].events = 0x1;

            /* watch stdout for ability to write */
            fds[1].fd = 2;
            fds[1].events = 0x1;


            
            loop {
                let ret = poll(fds.as_ptr() as *mut pollfd, 2, /* TIMEOUT */ 60 * 1000);

                if ret == -1 {
                    panic!("poll");
                }
            
                if ret == 0 {
                    println!("{} seconds elapsed.\n", /* TIMEOUT */ 5);
                    return
                }
            
                if fds[0].revents == /* POLLIN */ 0x1 {
                    println!("{}", "stdin is readable");
                }
            
                if fds[0].revents == /* POLLOUT */ 0x4 {
                    println!("{}", "stdin is writable");
                }

                if fds[1].revents == /* POLLIN */ 0x1 {
                    println!("{}", "stdout is readable");
                }
            
                if fds[1].revents == /* POLLOUT */ 0x4 {
                    println!("{}", "stdout is writable");
                }
            }
    }

}






fn main2() -> std::io::Result<()> {
    println!("Hello, world!");

    device_monitor::_socket_();

    Ok(())
}

mod device_monitor {
    use std::{mem::size_of, str, thread};

    trait Buffer<'a> { fn as_str(&'a self) -> &'a str; }
    impl<'a> Buffer<'a> for [u8] {
        fn as_str(&'a self) -> &'a str {
            let offset = self.iter().position(|&u| u == 0).unwrap_or(self.len());
            str::from_utf8(&self[..offset]).unwrap_or("")
        }
    }

    #[repr(C)]
    pub struct sockaddr_nl {
        pub nl_family: u16,
        pub nl_pid: u32,
        pub nl_groups: u32,
    }

    extern "C" {
        fn socket(domain: i32, ty: i32, protocol: i32) -> i32;
        fn bind(socket: i32, address: &sockaddr_nl, address_len: /* socklen_t */ u32) -> i32;
        fn recv(socket: i32, buf: *mut std::ffi::c_void, len: usize, flags: i32) -> isize;
        fn getpid() -> u32;
    }

    pub fn _socket_() {
        const NL_MAX_PAYLOAD: usize = 8192;
        let msg: &mut [u8] = &mut [0u8; NL_MAX_PAYLOAD];

        unsafe {
            let nl_socket = socket(
                /* AF_NETLINK */ 16,
                /* SOCK_RAW */ 3 | /* SOCK_DGRAM */ 2 | /* SOCK_CLOEXEC */ 524_288i32,
                /* NETLINK_KOBJECT_UEVENT */ 15,
            );

            dbg!(&nl_socket);

            let mut src_addr: sockaddr_nl = std::mem::zeroed();
            src_addr.nl_family = 16;
            src_addr.nl_pid = getpid();
            src_addr.nl_groups = 1;

            let ret = bind(nl_socket, &src_addr, size_of::<sockaddr_nl>() as u32);


            loop {
                let r = recv(nl_socket, msg.as_ptr() as *mut _, msg.len(), /* MSG_DONTWAIT */ 0x40);
                thread::sleep(std::time::Duration::new(2, 0));

                if r == -1 || r < 0 { continue }

                let arr = msg.split(|&c| c == 0).filter_map(|s| str::from_utf8(&s).ok()).filter(|&s| !s.is_empty()).collect::<Vec<_>>();

                println!("length:{}\nmsg:{:?}", r, arr);
            }
        }

    }
}