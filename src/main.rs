use word::{Answer, Color, Guess};
mod reader;
mod word;
use colored::{ColoredString, Colorize};

const WORD_LENGTH: i32 = 5;
const MAX_ATTEMPTS: i32 = 6;

fn main() {
    /// TODO: ランダムに正解となる単語を決定する
    /// TODO: for文でMAX_ATTEMPS文ループ回す
    //  let input = reader::read_from_stdin();
    let content = reader::read_from_txt().unwrap();

    let answer = Answer::new("WORDLE");
    let guess = Guess::new("WISDOM");
    let word_vec = word::assert(&guess, &answer);
    for (i, val) in word_vec.iter().enumerate() {
        let char = &guess.internal_word().chars().nth(i).unwrap();
        let colored_str = val.decorate_word(&char.to_string());
        print!("{}", colored_str);
    }
}

fn display() {}
