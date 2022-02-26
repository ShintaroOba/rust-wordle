use std::{fmt::Result, fs};

fn main() {
    /// コンソールから探索文字を検索できるようにする
    /// コンソールのセッションをキープしたい

    print!("Hello");
    match read() {
        Ok(()) => println!("OK"),
        Err(_) => print!("NG"),
    }
}

fn read() -> Result {
    
    let content = fs::read_to_string("./dictionary/dictionary.txt");
    // 文字列を改行コード、カンマをトリム処理を実装
    println!("{:?}", content);
    Ok(())
}
