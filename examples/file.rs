extern crate aurora;
extern crate clap;
extern crate linefeed;

use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App, ArgMatches};
use aurora::config::{LogLevel, Config};
use aurora::Aurora;

fn main() {
    let matches = App::new("aurora")
        .version("0.1.0")
        .author("Noah Charlton <ncharlton002@gmail.com>")
        .about("Lua interpreter written in pure rust")
        .arg(Arg::with_name("verbose")
            .help("Sets the log level to verbose")
            .short("v")
            .long("verbose")
            .conflicts_with("quiet"))
        .arg(Arg::with_name("quiet")
            .help("Sets the log level to quiet")
            .short("q")
            .long("quiet")
            .conflicts_with("verbose"))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .required(true)
            .help("Which file to run in the assets folder"))
        .get_matches();

    let config = create_config(&matches);

    let file = matches.value_of("file").unwrap();
    println!("Running Lua Src file: {}\n", file);
    run_file(file, config);
}

fn create_config(matches: &ArgMatches) -> Config{
    let log_level = if matches.is_present("verbose"){
        LogLevel::Verbose
    }else if matches.is_present("quiet"){
        LogLevel::Quiet
    }else{
        LogLevel::Normal
    };
    
    Config::new(log_level)
}

fn run_file(name: &str, config: Config){
    let src = load_file(name);
    let mut aurora = Aurora::new(config);

    match aurora.run(src){
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
