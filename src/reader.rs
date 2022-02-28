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

/// The reader accepts input from stdin.
/// 
pub fn read_from_stdin() {//-> //Vec<String> {
    println!("Enter your guess word:");
    
    // receive a word from stdin as String. 
    io::stdin().read_line(&mut String::new()).expect("Input validation error.");


}
