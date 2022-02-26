use std::{fmt::Result, fs};

fn main() {
    print!("Hello");
    match read() {
        Ok(()) => println!("OK"),
        Err(_) => print!("NG"),
    }
}

fn read() -> Result {
    let content = fs::read_to_string("./dictionary/dictionary.txt");
    println!("{:?}", content);
    Ok(())
}
