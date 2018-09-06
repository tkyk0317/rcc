mod lexer;
mod token;
mod ast;

use std::process;
use token::Token;
use lexer::LexicalAnalysis;

/**
 * 演算子をアセンブラへ.
 */
fn operator(ope_token: &Token) {
    match *ope_token {
        Token::Multi =>  println!("  mull %edx"),
        Token::Plus => println!("  addl %edx, %eax"),
        Token::Minus => println!("  subl %edx, %eax"),
        _ => process::abort()
    }
}

/**
 * 式評価.
 */
fn expression(tokens: &LexicalAnalysis) {
    let mut ope_token = Token::Unknown;
    for t in tokens.get_tokens() {
        match t.get_token_type() {
            // 数値.
            Token::Number => {
                println!("  sub $4, %rsp");
                println!("  movl ${}, 0(%rsp)", t.get_token_value());

                // 各数値をレジスタへ.
                if ope_token != Token::Unknown {
                    println!("  movl 0(%rsp), %edx\n  add $4, %rsp");
                    println!("  movl 0(%rsp), %eax\n  add $4, %rsp");

                    // 各演算子評価.
                    operator(&ope_token);

                    // 演算結果をrspへ退避.
                    println!("  sub $4, %rsp\n  movl %eax, 0(%rsp)");
                }
            }
            // 加算/減算/乗算演算子.
            Token::Plus | Token::Minus | Token::Multi=> {
                ope_token = t.get_token_type();
            }
            _ => {
                println!("Not Support Token");
                process::abort();
            }
        }
    }
}

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

    // 演算実施.
    expression(&p);

    println!("  movl 0(%rsp), %eax");
    println!("  add $4, %rsp");
    println!("  pop %rbp");
    println!("  ret");
}
