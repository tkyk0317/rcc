/* 別の作成方法で実施するので一旦、コメントアウト
mod lexer;
mod token;
mod ast;
*/

/**
 * メイン関数.
 */
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    println!(".global main");
    println!("main:");
    println!("  mov ${num}, %eax", num=s.trim().parse::<i32>().unwrap());
    println!("  ret");

/* 別の作成方法で実施するので一旦、コメントアウト
    // 標準入力を字句解析
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut p = lexer::LexicalAnalysis::new(&s);
    p.next_token();

    // 抽象構文木作成.
    let mut l = ast::Ast::new(p.get_tokens());
    let ast = l.parse();
    println!("{:?}", ast);
*/
}
