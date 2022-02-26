use std::fs;

fn read() {
    let content = fs::read_to_string("../dictionary/dictionary.txt");
    println!("{}", content);
}