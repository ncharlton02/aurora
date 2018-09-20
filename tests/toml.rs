extern crate aurora;
extern crate toml;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;

use aurora::data::LuaData;

const ROOT_PATH : &'static str = "tests/toml";

#[derive(Debug, Deserialize)]
struct TestInfo{
    src : String,
    statements: Option<usize>,
    tokens: Option<usize>,
    test_variables: Option<TestVars>
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
    let test_info: TestInfo = load_toml(path);
    println!("Running toml test: {:#?}", test_info);

    let lua_src = load_file(test_info.src);
    let tokens = aurora::parser::scanner::scan(lua_src);

    if let Some(num) = test_info.tokens{
        assert_eq!(tokens.len(), num);
    }

    let mut stmts = aurora::parser::parse(tokens);

    if let Some(num) = test_info.statements{
        assert_eq!(num, stmts.len())
    }

    println!("--------- Running -------");
    let interpreter = aurora::interpreter::run(&mut stmts);
    println!("--------- Finished -------");

    if let Some(vars) = test_info.test_variables{
        assert_eq!(vars.names.len(), vars.values.len());

        for i in 0..vars.names.len(){
            let name = vars.names.get(i).unwrap();
            let value = vars.values.get(i).unwrap();
            let actual = interpreter.get_variable(name.to_string()).unwrap_or(&LuaData::Nil);

            assert_eq!(format!("{}", actual), format!("{}", value));
        }
    }
}



fn load_toml(path: PathBuf) -> TestInfo{
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