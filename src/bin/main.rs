extern crate aurora;
extern crate clap;
extern crate linefeed;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App, SubCommand, ArgMatches};
use linefeed::{Interface, ReadResult};
use aurora::interpreter::{Interpreter, function::Function};
use aurora::parser;
use aurora::error::LuaError;
use aurora::config::{LogLevel, Config};
use aurora::data::LuaData;
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
        .subcommand(SubCommand::with_name("file")
            .about("Runs a lua file")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("Which file to run in the assets folder")))
        .subcommand(SubCommand::with_name("console")
            .about("Runs a console version of aurora"))
        .get_matches();

    let config = create_config(&matches);

    if let Some(sub_matches) = matches.subcommand_matches("file") {
        if let Some(file) = sub_matches.value_of("file") {
            println!("Running Lua Src file: {}\n", file);
            run_file(file, config);
        }else{
            println!("file argument not found! Please see help screen for more info");
        }
    }

    if let Some(_) = matches.subcommand_matches("console") {
        run_console().expect("Failed to run console!");
    }
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

fn run_console() -> io::Result<()>{
    let mut intepreter = create_console_interpreter();
    let interface = Interface::new("demo")?;
    interface.set_prompt("aurora> ")?;

    while let ReadResult::Input(line) = interface.read_line()? {
        interface.add_history_unique(line.clone());
        match run_line(&mut intepreter, line){
            Err(errors) => {
                for e in errors{
                    println!("{}", e);
                }
            },
            _ => (),
        };
    }

    Ok(())
}

fn create_console_interpreter() -> Interpreter{
    let mut intepreter = Interpreter::new();

    intepreter.register_func("quit".to_string(), Function::Rust(|_, _| -> Result<Option<LuaData>, LuaError>{
        ::std::process::exit(0);
    }));

    intepreter
}

fn run_line(intepreter: &mut Interpreter, line: String) -> Result<(), Vec<LuaError>>{
    let tokens = parser::scanner::scan(line)?;
    let stmts = match parser::parse(tokens){
        Err(e) => return Err(vec![e]),
        Ok(x) => x,
    };

    for stmt in stmts{
        match intepreter.run_stmt(&mut stmt.clone()){
            Err(e) => return Err(vec![e]),
            _ => (),
        };
    }

    Ok(())
}