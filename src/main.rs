mod lexer;
mod token;
mod ast;
mod asm;
mod config;

use ast::Ast;
use asm::Asm;

/**
 * メイン関数.
 */
fn main() {
    // 標準入力を字句解析
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut p = lexer::LexicalAnalysis::new(&s);
    p.read_token();
    let mut ast = Ast::new(p.get_tokens());

    // アセンブラへ変換.
    let mut asm = Asm::new();
    asm.generate(&ast.parse());
    println!("{}", asm.get_inst());
}
