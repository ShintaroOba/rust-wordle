use colored::{Colorize, ColoredString};
/// An value object that holds a correct answer inside.
pub struct Answer {
    target_word: String,
}

impl Answer {
    // factory method
    pub fn new(target_word: &str) -> Self {
        Answer {
            target_word: target_word.to_string(),
        }
    }

    pub fn internal_word(&self) -> String {
        self.target_word.clone()
    }
}

pub enum Color {
    GREEN,
    YELLOW,
    GRAY,
}

impl Color {
    fn decorate_word(&self, str: &str) -> ColoredString{
        match self{
            GREEN =>str.on_truecolor(152, 216, 105).black(),
            YELLOW => str.on_truecolor(247, 225, 150).black(),
            GRAY =>str.on_truecolor(111, 114, 121).white()
        }
        
    }
}

/// An value object that holds a player's guess word inside.
pub struct Guess {
    guess_word: String,
}

impl Guess {
    // factory method
    pub fn new(guess_word: &str) -> Self {
        Guess {
            guess_word: guess_word.to_string(),
        }
    }

    pub fn internal_word(&self) -> String {
        self.guess_word.clone()
    }
}

// 探索を行うメソッド
pub fn assert(guess_word: &Guess, target_word: &Answer) {
    let guess_word = guess_word.internal_word();
    /// 各Charごとに一致を確認する。
    /// TODO: 各Indexごとに正解（〇）、不正解（×）、位置ずれ（△）を保持するObjectを作成する
    for x in guess_word.chars() {
        println!("word_array: {:?}", x);
    }
}
