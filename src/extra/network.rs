use libc::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtattr {
    pub rta_len: u16,
    pub rta_type: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifinfomsg {
    pub ifi_family: u8,
    pub __ifi_pad: u8,
    pub ifi_type: u16,
    pub ifi_index: i32,
    pub ifi_flags: u32,
    pub ifi_change: u32,
}

unsafe fn main_0() {
    // local addr struct
    let fd: i32 = socket(16 as i32, SOCK_RAW as i32, 0 as i32);

    if fd.is_negative() {
        println!(
            "Failed to create netlink socket: {}",
            std::io::Error::last_os_error()
        ); // message buffer
        return;
    }

    // message structure
    let mut local = std::mem::zeroed::<libc::sockaddr_nl>(); // set message buffer as io
    local.nl_family = 16;
    local.nl_groups = 1 | 0x10 | 0x40;
    local.nl_pid = libc::getpid() as u32;

    let mut buf = [0; 8192]; // set size

    let mut iov = std::mem::zeroed::<libc::iovec>();

    iov.iov_base = buf.as_mut_ptr() as *mut _; // set groups we interested in
    iov.iov_len = std::mem::size_of::<[i8; 8192]>(); // set out id using current process id

    // initialize protocol message header
    let mut msg = std::mem::zeroed::<libc::msghdr>();

    msg.msg_name = &mut local as *mut sockaddr_nl as *mut _; // io size
    msg.msg_namelen = std::mem::size_of::<sockaddr_nl>() as u32; // address size
    msg.msg_iov = &mut iov as _; // io vector
    msg.msg_iovlen = 1 as usize;


    if bind(
        fd,
        &mut local as *mut sockaddr_nl as *mut _,
        std::mem::size_of::<sockaddr_nl>() as u64 as socklen_t,
    ).is_negative() {
        // bind socket
        println!(
            "Failed to bind netlink socket: {}",
            std::io::Error::last_os_error()
        );
        libc::close(fd);
        return;
    }

    loop {
        // read and parse all messages from the
        let mut status = recvmsg(fd, &mut msg, /* MSG_DONTWAIT */ 0x40) as i32;

        //  check status
        if status < 0 {
            let err = std::io::Error::last_os_error();
            let errno = err.raw_os_error();

            if errno == Some(4) || errno == Some(11) {
                libc::usleep(250000 as u32);
            } else {
                println!("Failed to read netlink: {}", std::io::Error::last_os_error());
            }
        } else {
            
            // message parser
            let mut h: *mut nlmsghdr = buf.as_mut_ptr() as *mut nlmsghdr;

            let len = (*h).nlmsg_len as usize;



            let name = std::ffi::CStr::from_ptr(buf.as_mut_ptr().offset(std::mem::size_of::<nlmsghdr>() as isize + 20)).to_str().unwrap_or_default();

            
            if len > status as usize {
                println!("Invalid message length: {}", len);
            } else {

                // now we can check message type
                if (*h).nlmsg_type == RTM_NEWROUTE || (*h).nlmsg_type == RTM_DELROUTE {
                    println!("Routing table was changed"); // in other case we need to go deeper
                } else {

                    // structure for network interface info
                    let ifi = (h as *mut i8)
                        .offset(std::mem::size_of::<nlmsghdr>() as isize)
                        as *mut ifinfomsg; // get attributes

                    let if_upp = if (*ifi).ifi_flags & IFF_UP as u32 != 0 {
                        "UP"
                    } else {
                        "DOWN"
                    };



                    // dbg!(std::ffi::CStr::from_ptr((ifi as *mut i8).offset((std::mem::size_of::<ifinfomsg>() + std::mem::size_of::<rtattr>()) as isize) as *mut i8).to_str().unwrap_or_default());


                    

                    let length = std::mem::size_of::<ifinfomsg>() + std::mem::size_of::<rtattr>();
                    let rta = (ifi as *mut i8).offset(length as isize) as *mut i8;

                    let ifname = std::ffi::CStr::from_ptr(rta).to_str().unwrap_or_default();

                    let if_runn = if (*ifi).ifi_flags & IFF_RUNNING as u32 != 0 {
                        "RUNNING"
                    } else {
                        "NOT RUNNING"
                    };

                    match (*h).nlmsg_type as i32 {

                        // what is actually happenned?
                        21 => println!("Interface {}: address was removed", ifname),
                        // align offsets by the message length, this is important

                        17 => println!("Network interface {} was removed", ifname),
                        16 => println!("New network interface {}, state: {} {}", ifname, if_upp, if_runn),
                        20 => println!("Interface {}", ifname),

                        _ => {}
                    }
                }

                // h = (h as *mut i8).offset(len as isize) as *mut nlmsghdr
            }

            libc::usleep(250000 as u32);
        }
    }
}

pub fn main() {
    unsafe { main_0() };
}
