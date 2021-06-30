use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TYPE { KEY, VALUE, ELEMENT, TABLE }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VALUE { Str(String), Arr(Vec<String>) }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parsed(HashMap<String, VALUE>);


impl<'a> Parsed {
    pub fn new(path: &'a PathBuf) -> Self {
        let mut mapped = HashMap::new();
    
        if let Ok(file) = std::fs::File::open(path.as_path()) {
    
            let mut quote = 0;
    
            let wilds = ['\n', '\r', '\t', ' ', '\'', '=', '[', ']', ','];
    
            let mut key = String::new();
            let mut value = String::new();
            let mut element = String::new();
            let mut table = String::new();
    
            let mut current = TYPE::KEY;
    
            let mut array: Vec<String> = vec![];
    
            let mut bytes = std::io::Read::bytes(file);
            while let Some(Ok(b)) = bytes.next() {
                let c = b as char;
    
                if (quote % 2 != 0 || !wilds.contains(&c)) && c != '\'' {
                    match current {
                        TYPE::KEY => key.push(c),
                        TYPE::VALUE => value.push(c),
                        TYPE::ELEMENT => element.push(c),
                        TYPE::TABLE => table.push(c),
                    };
                } else {
                    /* Discarded: \n \r <space> ' */
                    
                    match c {
                        '\'' => quote += 1,
                        '=' => current = TYPE::VALUE,
    
                        '[' => if current == TYPE::VALUE {
                            current = TYPE::ELEMENT;
                            array.clear();
                        } else {
                            current = TYPE::TABLE;
                            table.clear();
                        }
                        
    
                        ']' | ',' if current == TYPE::ELEMENT => {
                            array.push(element.to_owned());
                            element.clear();
                        },
    
                        '\n' if (quote % 2 == 0 || current != TYPE::ELEMENT) => {
                            if !key.is_empty() {
                                let o = if value.is_empty() {
                                    VALUE::Arr(array.to_owned())
                                } else {
                                    VALUE::Str(value.to_owned())
                                };
    
                                if !table.is_empty() {
                                    key = table.to_owned() + "." + &key.to_owned();
                                }
    
                                mapped.insert(key.to_owned(), o);
                            }
    
                            key.clear();
                            value.clear();
                            current = TYPE::KEY
                        }
    
                        _ => ()
                    }
                }
            }
        }
    
        Self(mapped)
    }


    pub fn get_string(&'a self, k: &'a str) -> Option<&'a String> {
        match self.0.get(k) {
            Some(VALUE::Str(s)) => Some(s),
            _ => None
        }
    }

    pub fn get_array(&'a self, k: &'a str) -> Option<&'a Vec<String>> {
        match self.0.get(k) {
            Some(VALUE::Arr(s)) => Some(s),
            _ => None
        }
    }
}