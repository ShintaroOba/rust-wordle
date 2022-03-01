mod char_research;
mod reader;

const WORD_LENGTH: i32 = 5;
const MAX_ATTEMPTS: i32 = 6;
fn main() {
    // TODO: for文でMAX_ATTEMPS文ループ回す
    let input = reader::read_from_stdin();
    let content = reader::read_from_txt().unwrap();
}
