
use super::{Token, scanner};

#[test]
fn scan_tests(){
    verify_token_count(4, "print(\"Hello World!\"");
    verify_token_count(17, "name = John\nage = 5\nprint(name)\nprint(age)")
}

fn verify_token_count(num: usize, src: &str){
    let tokens = scanner::scan(src.to_string());

    assert_eq!(num, tokens.len());
}