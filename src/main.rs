extern crate aurora;

fn main() {
    let result = aurora::run("print(\"Hello World\")".to_string());

    println!("Result: {:?}", result);
}
