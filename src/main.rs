use rand::Rng;
use word::{Answer, Guess};
mod reader;
mod word;

const WORD_LENGTH: i32 = 5;
const MAX_ATTEMPTS: i32 = 6;

fn main() {
    /// TODO: ランダムに正解となる単語を決定する
    /// TODO: for文でMAX_ATTEMPS文ループ回す
    let mut input = String::new();

    while {
        // 改行コードをtrim
        input = reader::read_from_stdin().trim().to_string();
        input.len() != 5
    } {
        if input.len() != 5 {
            println!("Guess word must be 5 characters.");
        }
    }
    let guess = Guess::new(&input);

    let random_word = get_random_word();

    let answer = Answer::new(&random_word);
    println!("Answer word is: {:?}", &answer);

    let word_vec = word::assert(&guess, &answer);
    for (i, val) in word_vec.iter().enumerate() {
        // VOが保有する文字列からi文字目のcharを取得
        let char = &guess.internal_val().chars().nth(i).unwrap();
        let colored_str = val.decorate_word(&char.to_string());
        print!("{}", colored_str); // dont remove
    }
}

fn get_random_word() -> String {
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
