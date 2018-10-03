mod lexer;
mod token;
mod ast;
mod asm;
mod config;
mod string;
mod map;
mod symbol;

use ast::Ast;
use asm::Asm;
use symbol::Symbol;

#[doc = "メイン関数"]
fn main() {
    // 標準入力を字句解析
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut p = lexer::LexicalAnalysis::new(&s);
    p.read_token();

    // AST作成
    let mut symbol = Symbol::new();
    let mut ast = Ast::new(p.get_tokens(), &mut symbol);

    // アセンブラへ変換.
    let mut asm = Asm::new();
    asm.generate(&ast.parse());
    println!("{}", asm.get_inst());
}
