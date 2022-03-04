use std::process::exit;

use rand::Rng;
use word::{Answer, Guess};

use crate::word::Color;

mod reader;
mod word;

const WORD_LENGTH: usize = 5;
const MAX_ATTEMPTS: i32 = 6;

fn main() {
    let mut attemps = 0;
    let random_word = get_random_word();
    let answer = Answer::new(&random_word);
    while attemps < MAX_ATTEMPTS {

        #[warn(unused_assignments)]
        let mut input = String::new();
        while {
            // 改行コードをtrim
            print!("Enter your guess word {}/{}: ", attemps + 1, MAX_ATTEMPTS);
            input = reader::read_from_stdin().trim().to_string();
            input.len() != WORD_LENGTH
        } {
            if input.len() != WORD_LENGTH {
                println!("Guess word must be 5 characters.");
            }
        }
        let guess = Guess::new(&input);
        string_play(&guess, &answer);
        attemps += 1;
    }
    println!("Answer is: {:?}", answer.internal_val());
}

fn string_play(guess: &Guess, answer: &Answer) {
    let word_vec = word::assert(&guess, &answer);
    

    // word_vecのカラー通りに文字列を装飾
    for (i, val) in word_vec.iter().enumerate() {
        // VOが保有する文字列からi文字目のcharを取得
        let char = &guess.internal_val().chars().nth(i).unwrap();
        let colored_str = val.decorate_word(&char.to_string());
        print!("{}", colored_str); // dont remove
    }
    println!(""); // dont remove

    if word_vec.iter().all(|color| color == &Color::GREEN) {
        println!("Correct! Conguraturations!");
        exit(0);
    }
}
fn get_random_word() -> String {
    // read words from csv file
    let contents = reader::read_from_txt().unwrap();
    let contents_vec: Vec<String> = contents
        .rsplit(',')
        .into_iter()
        .map(|s: &str| s.to_string())
        .collect::<Vec<String>>();

    // vecサイズからから任意のIndexを指定し、その要素をvecから取得する
    let random_num: usize = rand::thread_rng().gen_range(0..contents_vec.len() as u32) as usize;
    let unwrap_str = contents_vec.get(random_num).unwrap();

    unwrap_str.to_string()
}
