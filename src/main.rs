mod lexer;
mod token;

/**
 * メイン関数.
 */
fn main() {
    // 標準入力を字句解析
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut p = lexer::LexicalAnalysis::new(&s);
    p.next_token();

    println!("{:?}", p.get_tokens());
}
