use colored::{ColoredString, Colorize};

/// An value object that holds a correct answer inside.
#[derive(Debug)]
pub struct Answer {
    target_word: String,
}

impl Answer {
    // factory method
    pub fn new(target_word: &str) -> Self {
        Answer {
            target_word: target_word.to_string().to_uppercase(),
        }
    }

    pub fn internal_word(&self) -> String {
        self.target_word.clone()
    }
}

#[derive(Debug)]
pub enum Color {
    GREEN,
    YELLOW,
    GRAY,
}

impl Color {
    // 文字のカラーリングをします。
    pub fn decorate_word(&self, str: &str) -> ColoredString {
        match self {
            Color::GREEN => Self::add_brank(&str).on_truecolor(152, 216, 105).black(),
            Color::YELLOW => Self::add_brank(&str).on_truecolor(247, 225, 150).black(),
            Color::GRAY => Self::add_brank(&str).on_truecolor(111, 114, 121).white(),
        }
    }

    pub fn add_brank(str: &str) -> String {
        String::from(" ") + &str.to_string() + &String::from(" ")
    }
}

#[derive(Debug)]
/// An value object that holds a player's guess word inside.
pub struct Guess {
    guess_word: String,
}

impl Guess {
    // factory method
    pub fn new(guess_word: &str) -> Self {
        Guess {
            guess_word: guess_word.to_string().to_uppercase(),
        }
    }

    pub fn internal_word(&self) -> String {
        self.guess_word.clone()
    }
}

// 探索を行うメソッド
pub fn assert(guess_word: &Guess, target_word: &Answer) -> Vec<Color> {
    let guess_word = guess_word.internal_word();

    let mut index = 0;
    let mut ret: Vec<Color> = vec![];
    for x in guess_word.chars() {
        let target = target_word.internal_word();

        // xがAnswerと位置・文字が等しい場合
        if x == target.chars().nth(index).unwrap() {
            ret.push(Color::GREEN);
        // xがtargetにContainの場合、黄色を格納
        } else if target.contains(&x.to_string()) {
            ret.push(Color::YELLOW);
        } else {
            ret.push(Color::GRAY);
        }

        index += 1;
    }
    ret
}
