extern crate aurora;
extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};

fn main() {
    let matches = App::new("aurora")
        .version("0.1.0")
        .author("Noah Charlton <ncharlton002@gmail.com>")
        .about("Lua interpreter written in pure rust")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("The name of the lua file")
            .takes_value(true))
        .get_matches();

    let file_name = matches.value_of("file");

    let src = if let Some(name) = file_name{
        load_file(name)
    }else{
        "print(\"Hello World\")".to_string()
    };

    match aurora::run(src){
        Ok(_) => (),
        Err(errors) => {
            for e in errors{
                println!("{}", e)
            }
        },
    }
}

fn load_file(name: &str) -> String{
    let path = format!("assets/{}.lua", name);

    let mut file = File::open(&path).expect(&format!("Unable to open lua source file: {}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    contents
}