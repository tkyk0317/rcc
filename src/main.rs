mod asm;
mod ast;
mod config;
mod lexer;
mod map;
mod string;
mod symbol;
mod token;
mod arch;

use asm::Asm;
use ast::AstGen;

#[doc = "メイン関数"]
fn main() {
    // 標準入力を字句解析
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut p = lexer::LexicalAnalysis::new(&s);
    p.read_token();

    // AST作成
    let mut ast_gen = AstGen::new(p.get_tokens());
    let ast_tree = ast_gen.parse();

    // アセンブラへ変換.
    let var_table = ast_gen.get_var_symbol_table();
    let func_table = ast_gen.get_func_symbol_table();
    let mut asm = Asm::new(&var_table, &func_table);
    asm.exec(&ast_tree);
    println!("{}", asm.get_inst());
}
