
use std::fs;
use std::io::prelude::*;
use std::fs::File;

#[test]
fn test(){
    let paths = fs::read_dir("assets").unwrap();

    for path in paths {
        let path = path.unwrap().path();

        println!("Testing file: {}", path.display());

        super::run(load_file(path.display().to_string()));
    }
}

fn load_file(path: String) -> String{
    let mut file = File::open(&path).expect(&format!("Unable to open lua source file: {}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    contents
}