mod lexer;
mod token;
/* 別の作成方法で実施するので一旦、コメントアウト
mod ast;
*/

use token::Token;

/**
 * メイン関数.
 */
fn main() {
    println!(".global main");
    println!("main:");
    println!("  push %rbp");
    println!("  mov %rsp, %rbp");

    // 標準入力を字句解析
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut p = lexer::LexicalAnalysis::new(&s);
    p.read_token();

    let tokens = p.get_tokens();
    println!("  sub $4, %rsp");
    println!("  movl ${}, 0(%rsp)", tokens[0].get_token_value());
    println!("  movl ${}, %edx", tokens[2].get_token_value());
    println!("  movl 0(%rsp), %eax");
    println!("  add $4, %rsp");

    // トークン種別に対応した命令を発行.
    if Token::Minus == tokens[1].get_token_type() {
        println!("  subl %edx, %eax");
    }
    else {
        println!("  addl %edx, %eax");
    }

    println!("  sub $4, %rsp");
    println!("  movl %eax, 0(%rsp)");
    println!("  movl 0(%rsp), %eax");
    println!("  add $4, %rsp");

    println!("  pop %rbp");
    println!("  ret");

/* 別の作成方法で実施するので一旦、コメントアウト
    // 抽象構文木作成.
    let mut l = ast::Ast::new(p.get_tokens());
    let ast = l.parse();
    println!("{:?}", ast);
*/
}
