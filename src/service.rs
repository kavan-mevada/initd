use core::str;
use std::{collections::HashMap, fs::{File, OpenOptions}, io::{Error, ErrorKind, Read, Result}, iter::FromIterator, ops::Deref, path::{PathBuf}, rc::Rc};

use crate::linux::{execve, fork};









pub struct ServiceManager {
    running: Vec<Rc<str>>
}

impl ServiceManager {
    pub fn init() -> Self { Self { running: Vec::new() } }
    pub fn run(&self, service: &Service) -> Result<isize> {
        let parent = PathBuf::from("./services");

        for req in service.requires.iter() {
            let path = parent.join(req.deref()).with_extension("service");
            let service = Service::new(&path)?;
            let ret = self.run(&service);
            if ret.is_err() { return Err(Error::from(ErrorKind::NotFound))? }
        }

        println!("Hello from child!: {}", service.fname);

        execve(&["/bin/sh", "-c", "aasdas"], &[])
    }
}



#[derive(Debug)]
pub struct Service<'a, T = Rc<str>> {
    name: T,
    program: Rc<[T]>,
    requires: Rc<[T]>,

    fname: &'a str
}

impl<'a> Default for Service<'a> {
    fn default() -> Self {
        let (str, arr, arr2) = ("", [], []);
        Self {
            name: Rc::from(str),
            program: Rc::from(arr),
            requires: Rc::from(arr2),

            fname: str
        }
    }
}


impl<'a> Service<'a> {
    pub(crate) fn new(path: &'a PathBuf) -> Result<Self> {
        let mut _self = Self::default();

        _self.fname = path
            .file_name()
                .map(std::ffi::OsStr::to_str)
                .map(Option::unwrap_or_default)
                    .unwrap_or_default();

        let file = OpenOptions::new()
            .read(true).open(path)?;

        Self::open(file, |key, value| {
            match key {
                "service.Label" => _self.name = Rc::from(value),
                "service.Requires" => _self.requires = value.split('\'')
                                        .map(Rc::from).collect(),
                _ => ()
            }
        });

        Ok(_self)
    }

    pub fn open<R: Read, F>(reader: R, mut f: F) where F: FnMut(&str, &str) {
        let mut bytes = std::io::Read::bytes(reader);

        let wildcards = ['\n', '\r', '\t', ' ', '[', ']'];

        let (mut offset, mut quote) = (0, 0);
        let mut buffer = String::new();

        loop {
            let b = bytes.next();
    
            if b.is_none() { break }
    
            if let Some(Ok(b)) = b {
                let mut c = b as char;
    
                if c as char == '\'' {
                    quote += 1;
                    continue;
                }
        
                if quote % 2 != 0 || (quote % 2 == 0 && (c == ',' || !wildcards.contains(&c))) {
                    if quote % 2 == 0 && c == ',' { c = '\'' }
                    buffer.push(c);
                }
    
                if quote % 2 != 0 || c != '\n' { continue }
            }
    
    
            if let Some((key, value)) = buffer.split_once('=') {
                f(key, value);
                buffer.drain(offset..);
            } else if buffer.len() > 0 {
                if offset != buffer.len() { buffer.drain(..offset); buffer.push('.') }
                offset = buffer.len()
            }
        }
    }
}



#[test]
fn parse_service_file() {
    let data = std::io::Cursor::new(b"
ip = '127.0.1.1'

[keys.subkeys]
github = 'xxxxxxxxx
xxxxxxxx'


travis = 'yyyyyyyy
yy yyyyyyy'
dependencies = [ 'x,x', 'y
yyy
yysy', 'zz zzz']

[hello]
twitter = 'id'");

    // open(data, |key, value| {
    //     dbg!(key);
    //     match key {
    //         "ip" => assert_eq!(value, "127.0.1.1"),
    //         "keys.subkeys.github" => assert_eq!(value, "xxxxxxxxx\nxxxxxxxx"),
    //         "keys.subkeys.travis" => assert_eq!(value, "yyyyyyyy\nyy yyyyyyy"),
    //         "keys.subkeys.dependencies" => assert_eq!(value, "x,x'y\nyyy\nyysy'zz zzz"),
    //         "hello.twitter" => assert_eq!(value, "id"),
    //         _ => assert!(false, "malformed data!")
    //     };
    // });
}