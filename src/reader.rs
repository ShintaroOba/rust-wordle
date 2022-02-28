use std::{fs, io};


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
pub fn read_from_stdin() -> String {
    println!("Enter your guess word:");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Input validation error.");
    input

}
