use std::{fs, io};

const WORD_LENGTH: i32 =5; 
const MAX_ATTEMPTS: i32 = 6;

/// The reader retrieve all words from csv-file.
/// 
pub fn read_from_txt() -> Result<String, io::Error> {
    let content = match fs::read_to_string("./dictionary/wordle_words.csv"){
        Ok(content) => content,
        Err(e) => return Err(e)
    };

    Ok(content)
}

/// The reader accepts input from std-in.
/// 
pub fn read_from_stdin() {//-> //Vec<String> {
    let mut input = String::new();
    print!("Enter your guess word:");
    io::stdin().read_line(&mut input).expect("Input validation error.");
    println!("input val: {}", input);


}
