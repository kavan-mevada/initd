use std::{collections::HashMap, path::{Path, PathBuf}};

enum VALUE {
    Arr(Vec<String>),
    Str(String),
}

#[derive(Debug, PartialEq)]
enum TYPE {
    KEY,
    VALUE,
    ELEMENT,
    TABLE,
}

fn parse_from<P: AsRef<Path>>(path: P) {
    let file = std::fs::File::open(path).unwrap();


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

                        //mapped.insert(key.to_owned(), o);
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