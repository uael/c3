extern crate c3;
use std::env;
use std::path::Path;

fn main() {
    let file = env::args().nth(1);
    let file = file.as_ref().map(|s|s.as_ref()).unwrap_or("test.c");
    match c3::C3::new().parse_file(Path::new(file), &[]) {
        Ok(tu) => println!("{:#?}", tu),
        Err(err) => println!("ERROR: {}", err),
    };
}
