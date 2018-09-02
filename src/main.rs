mod lexer;
mod token;
/* 別の作成方法で実施するので一旦、コメントアウト
mod ast;
*/

use std::process;
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

    // 加減算実施.
    let mut operation_token = Token::Unknown;
    for t in p.get_tokens() {
        match t.get_token_type() {
            // 数値.
            Token::Number => {
                if operation_token == Token::Unknown {
                    println!("  sub $4, %rsp");
                    println!("  movl ${}, 0(%rsp)", t.get_token_value());
                }
                else {
                    println!("  movl ${}, %edx", t.get_token_value());
                    println!("  movl 0(%rsp), %eax");
                    println!("  add $4, %rsp");

                    // 演算子を評価.
                    if Token::Plus == operation_token {
                        println!("  addl %edx, %eax");
                    }
                    else {
                        println!("  subl %edx, %eax");
                    }
                    println!("  sub $4, %rsp");
                    println!("  movl %eax, 0(%rsp)");
                }
            }
            // 加算/減算演算子.
            Token::Plus | Token::Minus => {
                operation_token = t.get_token_type();
            }
            _ => {
                process::abort();
            }
        }
    }

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
