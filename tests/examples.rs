extern crate aurora;

use std::fs;
use std::io::prelude::*;
use std::fs::File;

use aurora::{Aurora, config::*};

#[test]
fn run_lua_examples(){
    let paths = fs::read_dir("assets").unwrap();

    for path in paths {
        let path = path.unwrap().path();

        if path.is_dir(){
            continue;
        }

        let mut aurora = Aurora::new(Config::new(LogLevel::Verbose));

        println!("Testing file: {}", path.display());

        match aurora.run(load_file(path.display().to_string())){
            Ok(_) => (),
            Err(errors) => {
                for e in errors{
                    println!("{}", e)
                }

                panic!();
            },
        }
    }
}

fn load_file(path: String) -> String{
    let mut file = File::open(&path).expect(&format!("Unable to open lua source file: {}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    contents
}