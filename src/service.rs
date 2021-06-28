use core::str;
use std::{ops::Deref, path::Path, rc::Rc};


#[derive(Debug, Clone)]
pub struct Service {
    pub Label: Rc<str>,
    pub Program: Rc<[Rc<str>]>,
    pub Requires: Rc<[Rc<str>]>,
    pub Wants: Rc<[Rc<str>]>,
    pub BroadcastDomain: Rc<str>,
    pub SuccessCode: usize,

    pub OnJobBroadcast: Option<Rc<str>>,
    pub OnExitCode: Option<Rc<str>>,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            Label: Rc::from(""),
            BroadcastDomain: Rc::from(""),
            SuccessCode: 0,
            Program: Rc::from([]),
            Requires: Rc::from([]),
            Wants: Rc::from([]),

            OnJobBroadcast: None,
            OnExitCode: None,
        }
    }
}

impl<'a> Service {

    pub fn from<'b, P>(path: P) -> Self where P: AsRef<Path> {
        let mut _self = Service::default();

        Self::open(path, |key, value| {
            match key.deref() {
                "service.Label" => { _self.Label = value },
                "service.Program" => { _self.Program = value.split('\'').map(Rc::from).collect() }
                "service.Requires" => { _self.Requires = value.split('\'').map(Rc::from).collect() }
                "service.Wants" => { _self.Wants = value.split('\'').map(Rc::from).collect() }
                "service.BroadcastDomain" => { _self.BroadcastDomain = value }
                "service.SuccessCode" => { _self.SuccessCode = value.parse().unwrap_or(0) }

                "alive-on.JobBroadcast" => { _self.OnJobBroadcast = Some(value) }
                "alive-on.ExitCode" => { _self.OnExitCode = Some(value) }
                _ => ()
            }
        });

        _self
    }

    fn open<'b, P, F>(path: P, mut f: F) where P: AsRef<Path>, F: FnMut(Rc<str>, Rc<str>) {
    
        let file = std::fs::File::open(path.as_ref()).expect("Error opening file!");
        let mut bytes = std::io::Read::bytes(file);
    
        let wildcards = ['\n', '\r', '\t', ' ', '[', ']'];
    
        let mut buffer = String::new();
        let mut offset = 0usize;
    
        let mut quote = 0;
        while let Some(Ok(b)) = bytes.next() {
            let mut c = b as char;
    
            if c == '\'' {
                quote += 1;
                continue;
            }
    
            if quote % 2 != 0 || (quote % 2 == 0 && (c == ',' || !wildcards.contains(&c))) {
                if quote % 2 == 0 && c == ',' { c = '\'' }
                buffer.push(c);
            }
    
            if quote % 2 == 0 && c == '\n' {
                if let Some((key, value)) = buffer.split_once('=') {
                    f(Rc::from(key), Rc::from(value));
                    buffer.drain(offset..);
                } else if buffer.len() > 0 {
                    if offset != buffer.len() { buffer.drain(..offset); buffer.push('.') }
                    offset = buffer.len()
                }
            }
        }
    }
}