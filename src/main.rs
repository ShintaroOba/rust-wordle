use colored::{ColoredString, Colorize};
use word::{Answer, Guess};
mod reader;
mod word;

const WORD_LENGTH: i32 = 5;
const MAX_ATTEMPTS: i32 = 6;

fn main() {
    /// TODO: ランダムに正解となる単語を決定する
    /// TODO: for文でMAX_ATTEMPS文ループ回す
    //  let input = reader::read_from_stdin();
    let content = reader::read_from_txt().unwrap();

    let answer = Answer::new("WORDLE");
    let guess = Guess::new("GUESS");
    word::assert(&guess, &answer);

    // GREEN
    print!("{}", " W ".on_truecolor(152, 216, 105).black());
    print!("{}", " O ".on_truecolor(152, 216, 105).black());

    // YELLOW
    print!("{}", " R ".on_truecolor(247, 225, 150).black());
    print!("{}", " D ".on_truecolor(247, 225, 150).black());
    // GRAY
    print!("{}", " L ".on_truecolor(111, 114, 121).white());
    print!("{}", " E ".on_truecolor(111, 114, 121).white());
}

fn display(){
    
}