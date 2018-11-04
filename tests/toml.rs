extern crate aurora;
extern crate toml;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;

use aurora::data::LuaData;
use aurora::interpreter::Interpreter;
use aurora::Token;
use aurora::Stmt;
use aurora::parser::Parser;
use aurora::parser::scanner::Scanner;

const ROOT_PATH : &'static str = "tests/toml";

#[derive(Debug, Deserialize)]
struct TestInfo{
    src : String,
    statements: usize,
    tokens: usize,
    line_count: usize,
    test_variables: TestVars
}

#[derive(Debug, Deserialize)]
struct TestVars{
    names: Vec<String>,
    values: Vec<String>
}

#[test]
fn run_toml_tests(){
    let toml_paths = get_toml_paths();

    for path in toml_paths{
        run_toml_test(path);
    }
}

fn run_toml_test(path: PathBuf){
    let test_info: TestInfo = load_toml(&path);
    println!("Running toml test: {:#?}", test_info);

    let lua_src = load_file(test_info.src);
    let (scanner, tokens) = create_scanner(lua_src);
    println!("Tokens: \n{:#?}", tokens);

    assert_eq!(tokens.len(), test_info.tokens, "Token count does not match");
    assert_eq!(scanner.line_num, test_info.line_count, "Scanner line count does not match");
    

    let (parser, stmts) = create_parser(tokens);
    println!("Stmts: \n{:#?}", stmts);

    let stmt_count = aurora::count_stmts_recur(&stmts);
    assert_eq!(test_info.statements, stmt_count as usize, "Stmt count does not match");
    assert_eq!(parser.line, test_info.line_count, "Parser line count does not match");
    

    let interpreter = create_interpreter(stmts);
    check_variables(test_info.test_variables, interpreter);
}

fn check_variables(vars: TestVars, interpreter: Interpreter){
    assert_eq!(vars.names.len(), vars.values.len());

    for i in 0..vars.names.len(){
        let name = vars.names.get(i).unwrap();
        let value = vars.values.get(i).unwrap();
        let actual = interpreter.get_variable(name.to_string()).unwrap_or(&LuaData::Nil);

        assert_eq!(format!("{}", actual), format!("{}", value));
    }
}

fn create_scanner(src: String) -> (Scanner, Vec<Token>){
    let mut scanner = Scanner::new(src);    
    let tokens = match scanner.scan(){
        Ok(x) => x,
        Err(errors) => {
            for e in errors{
                println!("{}", e);
            }

            panic!();
        },
    };

    return (scanner, tokens);
}

fn create_parser(tokens: Vec<Token>) -> (Parser, Vec<Stmt>){
    let mut parser = Parser::new(tokens);

    let stmts = match parser.parse(){
        Ok(x) => x,
        Err(e) => {
            panic!("{}", e);
        }
    };

    return (parser, stmts);
}

fn create_interpreter(mut stmts: Vec<Stmt>) -> Interpreter{
    let mut interpreter = Interpreter::new();

    println!("--------- Running -------");
    for mut stmt in stmts.iter_mut(){
        match interpreter.run_stmt(stmt){
            Ok(x) => x,
            Err(e) => {
                println!("{}", e);
                panic!();
            }
        };
    }
    println!("--------- Finished -------");

    return interpreter;
}

fn load_toml(path: &PathBuf) -> TestInfo{
    let toml_str = load_file(path.to_str().unwrap().to_string());
    toml::from_str(&toml_str).unwrap()
}

fn load_file(path: String) -> String{
    let mut file = File::open(&path).expect(&format!("Unable to open lua source file: {}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    contents
}

fn get_toml_paths() -> Vec<PathBuf>{
    let paths = fs::read_dir(ROOT_PATH).unwrap();
    let mut buffers = Vec::new();

    for path in paths {
        buffers.push(path.unwrap().path().to_path_buf());
    }

    buffers
}