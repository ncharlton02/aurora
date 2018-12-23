extern crate aurora;

use aurora::*;
use aurora::config::*;
use aurora::interpreter::*;

#[test]
#[should_panic]
fn duplicate_load_modules_fails_test() {
    let mut interpreter = Interpreter::new();
    interpreter.load_module("foo".to_string(), Vec::new()).unwrap();
    interpreter.load_module("foo".to_string(), Vec::new()).unwrap();
}

#[test]
#[should_panic]
fn module_doesnt_load_twice_test() {
        //Core library will fail if loaded twice. This test is simply to guarentee that test case happens
    let mut aurora = Aurora::new(Config::new(LogLevel::Normal));
    aurora.run(r#"require("lib/core");require("lib/core")"#.to_string()).unwrap();
}