mod lexer;
mod token;
mod ast;

use std::process;
use ast::Ast;
use ast::Expr;

/**
 * 演算子をアセンブラへ.
 */
fn operator(ope: &Expr) {
    match *ope {
        Expr::Multiple(_, _) => println!("  mull %edx"),
        Expr::Plus(_, _)     => println!("  addl %edx, %eax"),
        Expr::Minus(_, _)    => println!("  subl %edx, %eax"),
        _ => process::abort()
    }
}

/**
 * 式評価.
 */
fn expression(ast: &Expr) {
    match *ast {
        Expr::Plus(ref a, ref b) |
        Expr::Minus(ref a, ref b) |
        Expr::Multiple(ref a, ref b) => {
            expression(a);
            expression(b);

            // 各演算子評価.
            println!("  movl 0(%rsp), %edx\n  add $4, %rsp");
            println!("  movl 0(%rsp), %eax\n  add $4, %rsp");
            operator(ast);

            // 演算結果をrspへ退避.
            println!("  sub $4, %rsp\n  movl %eax, 0(%rsp)");
        }
        Expr::Factor(a) => {
            // 数値.
            println!("  sub $4, %rsp");
            println!("  movl ${}, 0(%rsp)", a);
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
    let mut ast = Ast::new(p.get_tokens());

    // 演算実施.
    expression(&ast.parse());

    println!("  movl 0(%rsp), %eax");
    println!("  add $4, %rsp");
    println!("  pop %rbp");
    println!("  ret");
}
