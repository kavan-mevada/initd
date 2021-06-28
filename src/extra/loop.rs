// mod parser;
// use parser::Parsed;

use std::{path::{PathBuf}};

fn main() -> std::io::Result<()> {
    let path = PathBuf::from("etc/initd/sshd.service");
    let service = Service::new(&path);
    service.run();

    // let f = read_dir("path");

    // let travis = data.get_string("metadata.name");
    // let aa = data.get_array("service.requires");

    // println!("{:?} {:?}", travis, aa);
    

    Ok(())
}



struct Service<'a> {
    path: &'a PathBuf
}

impl<'a> Service<'a> {
    fn new(path: &'a PathBuf) -> Self {
        Self { path }
    }

    fn run<'b>(&'b self) -> bool {
        let parent = self.path.parent().unwrap();
        if !self.path.exists() { return false }

        let data = Parsed::new(self.path);

        for &typ in &["requires", "wants"] {
            if let Some(requires) = data.get_array(&["service.", typ].concat()) {
                for dependency in requires {
                    let path = &parent
                        .join(dependency.to_owned() + ".service") ;
                    if (Service { path }).run() == false && typ == "requires" { return false }
                }
            }
        }

        //
        // Current service execution
        let name = &data.get_string("metadata.name");
        dbg!(name);


        true
    }
}