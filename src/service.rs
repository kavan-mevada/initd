use core::str;
use std::{collections::HashMap, fs::File, io::Read, path::{PathBuf}, rc::Rc};


#[derive(Debug)]
pub enum Node { Empty, NonEmpty(i32, Rc<Node>) }





#[derive(Debug, Clone)]
pub struct Service {
    data: HashMap<Rc<str>, Rc<str>>
}

impl Service {
    pub fn new(path: &PathBuf) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut mapped: HashMap<Rc<str>, Rc<str>> = HashMap::new();
        open(file, |key, value| { mapped.entry(Rc::from(key)).or_insert(Rc::from(value)); });
        Ok(Self { data: mapped })
    }
}

// Main parser
pub(crate) fn open<R, F>(reader: R, mut f: F) where R: Read, F: FnMut(&str, &str) {
    let mut bytes = std::io::Read::bytes(reader);

    let wildcards = ['\n', '\r', '\t', ' ', '[', ']'];

    let mut buffer = String::new();
    let mut offset = 0usize;

    let mut quote = 0;
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

    open(data, |key, value| {
        dbg!(key);
        match key.deref() {
            "ip" => assert_eq!(value, "127.0.1.1"),
            "keys.subkeys.github" => assert_eq!(value, "xxxxxxxxx\nxxxxxxxx"),
            "keys.subkeys.travis" => assert_eq!(value, "yyyyyyyy\nyy yyyyyyy"),
            "keys.subkeys.dependencies" => assert_eq!(value, "x,x'y\nyyy\nyysy'zz zzz"),
            "hello.twitter" => assert_eq!(value, "id"),
            _ => assert!(false, "malformed data!")
        };
    });
}








// Main parser
pub(crate) fn open2<'b, R, F>(reader: R, mut f: F) where R: Read, F: FnMut(&str, &str) + 'b {
    let mut bytes = std::io::Read::bytes(reader);

    let wildcards = ['\n', '\r', '\t', ' ', '[', ']'];

    let mut buffer = String::new();
    let mut offset = 0usize;

    let mut quote = 0;
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

