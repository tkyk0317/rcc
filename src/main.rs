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

#[cfg(test)]
mod test {
    use super::*;
    use std::process::Command;
    use std::fs;
    use std::io::{BufWriter, Write};

    // テスト用データ構造体
    struct TestData<'a> {
        inst: &'a str,
        ex_ret: i32,
    }

    // 評価関数.
    //
    // 引数で指定された文字列をコンパイル→実行、exitコードを返す
    fn eval(inst: &String) -> i32 {
        let mut p = lexer::LexicalAnalysis::new(inst);
        p.read_token();

        // AST作成
        let mut ast_gen = AstGen::new(p.get_tokens());
        let ast_tree = ast_gen.parse();

        // アセンブラへ変換.
        let var_table = ast_gen.get_var_symbol_table();
        let func_table = ast_gen.get_func_symbol_table();
        let mut asm = Asm::new(&var_table, &func_table);
        asm.exec(&ast_tree);

        // gccを使用して実行.
        {
            BufWriter::new(fs::File::create("test.s").unwrap()).write_all(asm.get_inst().as_bytes()).unwrap();
        }
        let _ = Command::new("gcc").args(&["-g3", "test.s", "-o", "test"]).output();
        Command::new("./test").status().unwrap().code().unwrap()
    }

    #[test]
    fn test_integration() {
        // テスト用データ
        [
            TestData { inst: "int main() { return 1+2; }", ex_ret: 3 },
            TestData { inst: "int main() { return 10+20; }", ex_ret: 30 },
            TestData { inst: "int main() { return 100+101; }", ex_ret: 201 },
            TestData { inst: "int main() { return 20-1; }", ex_ret: 19 },
            TestData { inst: "int main() { return 20-10; }", ex_ret: 10 },
            TestData { inst: "int main() { return 20-19; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1+2+3; }", ex_ret: 6 },
            TestData { inst: "int main() { return 1+2-3; }", ex_ret: 0 },
            TestData { inst: "int main() { return 100-2-3; }", ex_ret: 95 },
            TestData { inst: "int main() { return 2*4; }",  ex_ret: 8 },
            TestData { inst: "int main() { return 3*4; }", ex_ret: 12 },
            TestData { inst: "int main() { return 5*2*3; }", ex_ret: 30 },
            TestData { inst: "int main() { return 5*2*3-10; }", ex_ret: 20 },
            TestData { inst: "int main() { return 5+2*3; }", ex_ret: 11 },
            TestData { inst: "int main() { return 2*3+3*4; }", ex_ret: 18 },
            TestData { inst: "int main() { return (12+16); }", ex_ret: 28 },
            TestData { inst: "int main() { return (29-16); }", ex_ret: 13 },
            TestData { inst: "int main() { return (12+16)+3; }", ex_ret: 31 },
            TestData { inst: "int main() { return 3+(12+16); }", ex_ret: 31 },
            TestData { inst: "int main() { return (10+4)*10; }", ex_ret: 140 },
            TestData { inst: "int main() { return 10*(10+4); }", ex_ret: 140 },
            TestData { inst: "int main() { return 10/5; }", ex_ret: 2 },
            TestData { inst: "int main() { return 20/5/2; }", ex_ret: 2 },
            TestData { inst: "int main() { return 20/3; }", ex_ret: 6 },
            TestData { inst: "int main() { return 2+20/3; }", ex_ret: 8 },
            TestData { inst: "int main() { return 20/3+3; }", ex_ret: 9 },
            TestData { inst: "int main() { return 20%3; }", ex_ret: 2 },
            TestData { inst: "int main() { return 10+20%3; }", ex_ret: 12 },
            TestData { inst: "int main() { return 2==2; }", ex_ret: 1 },
            TestData { inst: "int main() { return 2+2==2*2; }", ex_ret: 1 },
            TestData { inst: "int main() { return 20/10==2; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1==2; }", ex_ret: 0 },
            TestData { inst: "int main() { return 2!=2; }", ex_ret: 0 },
            TestData { inst: "int main() { return 2+2!=2*2; }", ex_ret: 0 },
            TestData { inst: "int main() { return 20/10!=2; }", ex_ret: 0 },
            TestData { inst: "int main() { return 1!=2; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1>2; }", ex_ret: 0 },
            TestData { inst: "int main() { return 1<2; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1+3-1>2*4; }", ex_ret: 0 },
            TestData { inst: "int main() { return 1*3+20>4*2/2; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1>=2; }", ex_ret: 0 },
            TestData { inst: "int main() { return 1>=1; }", ex_ret: 1 },
            TestData { inst: "int main() { return 2>=1; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1<=2; }", ex_ret: 1 },
            TestData { inst: "int main() { return 2<=2; }", ex_ret: 1 },
            TestData { inst: "int main() { return 2<=3; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1+3-1>=2*4; }", ex_ret: 0 },
            TestData { inst: "int main() { return 1+3-1+5>=2*4; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1*3+20>=4*2/2; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1*3+20>=23; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1+3-1<=2*4; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1+3-1+5<=2*4; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1*3+20<=4*2/2; }", ex_ret: 0 },
            TestData { inst: "int main() { return 1*3+20<=23; }", ex_ret: 1 },
            TestData { inst: "int main() { return 3*(2+2) >= 4+(3*1); }", ex_ret: 1 },
            TestData { inst: "int main() { return 1&&1; }", ex_ret: 1},
            TestData { inst: "int main() { return 0&&1; }", ex_ret: 0 },
            TestData { inst: "int main() { return (1 + 1) && (2 * 1); }", ex_ret: 1 },
            TestData { inst: "int main() { return 1 == 1 && 2 < 1; }", ex_ret: 0 },
            TestData { inst: "int main() { return 4 / 2 == 0 + 2 && 2 > 1; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1||1; }", ex_ret: 1 },
            TestData { inst: "int main() { return 0||0; }", ex_ret: 0 },
            TestData { inst: "int main() { return (1 + 1) || (2 * 1); }", ex_ret: 1 },
            TestData { inst: "int main() { return 1 != 1 || 2 < 1; }", ex_ret: 0 },
            TestData { inst: "int main() { return 4 / 2 == 0 + 2 || 2 < 1; }", ex_ret: 1 },
            TestData { inst: "int main() { return (1 == 0 && 1) && (2 < 1 || 0); }", ex_ret: 0 },
            TestData { inst: "int main() { return 2 ? 1 : 3; }", ex_ret: 1 },
            TestData { inst: "int main() { return 2 > 1 ? 1 : 3; }", ex_ret: 1 },
            TestData { inst: "int main() { return 2 < 1 ? 1 : 3; }", ex_ret: 3 },
            TestData { inst: "int main() { return 2 > 1 ? (2 ? 10 : 100) : 3; }", ex_ret: 10 },
            TestData { inst: "int main() { return 2 == 1 ? (2 == 2 ? 9 : 99) : (0 ? 10 : 100); }", ex_ret: 100 },
            TestData { inst: "int main() { return +2; }", ex_ret: 2 },
            TestData { inst: "int main() { return 5 + (-5); }", ex_ret: 0 },
            TestData { inst: "int main() { return 3 - + - + - + - 2; }", ex_ret: 5 },
            TestData { inst: "int main() { return !2; }", ex_ret: 0 },
            TestData { inst: "int main() { return !(2 + 2 == 3 * 4); }", ex_ret: 1 },
            TestData { inst: "int main() { return !(2 != 3); }", ex_ret: 0 },
            TestData { inst: "int main() { return 2<<1; }", ex_ret: 4 },
            TestData { inst: "int main() { return 2>>1; }", ex_ret: 1 },
            TestData { inst: "int main() { return 2<<1<<1; }", ex_ret: 8 },
            TestData { inst: "int main() { return 8>>1>>1; }", ex_ret: 2 },
            TestData { inst: "int main() { return 2<<3; }", ex_ret: 16 },
            TestData { inst: "int main() { return 16>>2; }", ex_ret: 4 },
            TestData { inst: "int main() { return 5>>1; }", ex_ret: 2 },
            TestData { inst: "int main() { return 1&1; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1&0; }", ex_ret: 0 },
            TestData { inst: "int main() { return 1|0; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1|1; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1^1; }", ex_ret: 0 },
            TestData { inst: "int main() { return 0^1; }", ex_ret: 1 },
            TestData { inst: "int main() { return 0^0; }", ex_ret: 0 },
            TestData { inst: "int main() { return 1&0|1; }", ex_ret: 1 },
            TestData { inst: "int main() { return 183&109; }", ex_ret: 37 },
            TestData { inst: "int main() { return 183|109; }", ex_ret: 255 },
            TestData { inst: "int main() { return 183^109; }", ex_ret: 218 },
            TestData { inst: "int main() { return ~183 & 255; }", ex_ret: 72 },
            TestData { inst: "int main() { 2+2; return 1+2; }", ex_ret: 3 },
            TestData { inst: "int main() { 5>>1; return 1 != 2; }", ex_ret: 1 },
            TestData { inst: "int main() { int x; x = 4; x = x * x + 1; x = x + 3; return x; }", ex_ret: 20 },
            TestData { inst: "int main() { int x; x = 2 * 3 * 4; return x; }", ex_ret: 24 },
            TestData { inst: "int main() { int x; x = x = x = 3; return x; }", ex_ret: 3 },
            TestData { inst: "int test() { return 1; } int main() { return test(); }", ex_ret: 1 },
            TestData { inst: "int test() { int a; a = 1; return a + 19;} int main() { return test(); }", ex_ret: 20 },
            TestData { inst: "int test() { return 1; } int main() { test(); return 10; }", ex_ret: 10 },
            TestData { inst: "int test(int a) { return a + 1; } int main() { return test(1); }", ex_ret: 2 },
            TestData { inst: "int test(int a) { a = a * 2; return a + 10; } int main() { int b; b = 10; return test(b); }", ex_ret: 30 },
            TestData { inst: "int main() { int a; a = 0; if (10 == 10) { a = 2; a = a * 9; } return a; }", ex_ret: 18 },
            TestData { inst: "int main() { if (10 != 10) { int a; a = 2; a * 9; } return 2; }", ex_ret: 2 },
            TestData { inst: "int main() { if (2 == 10) { int a; a = 2; a * 9; } return 11; }", ex_ret: 11 },
            TestData { inst: "int main() { int a; a = 0; if (1 != 10) { a = 3; a = a + 9; } return a; }", ex_ret: 12 },
            TestData { inst: "int main() { if (1 == 10) { return 9; } else { return 4; } }", ex_ret: 4 },
            TestData { inst: "int main() { int a; a = 0; while (a < 1) { a = a + 1; } return a; }", ex_ret: 1 },
            TestData { inst: "int main() { int a; a = 0; while (a < 2) { a = a + 1; } return a; }", ex_ret: 2 },
            TestData { inst: "int main() { int a; a = 0; while (a <= 2) { a = a + 1; } return a; }", ex_ret: 3 },
            TestData { inst: "int main() { int a; a = 8; int b; b = 1; a = a + b; return a; }", ex_ret: 9 },
            TestData { inst: "int main() { int i; i = 0; for (i = 0 ; i < 2 ; i = i + 1) {;} return 11; }", ex_ret: 11 },
            TestData { inst: "int main() { int a; a = 0; int i; i = 0; for (i = 0 ; i < 10 ; i = i + 1) { a = a + 1;} return a; }", ex_ret: 10 },
            TestData { inst: "int test(int a, int b) { return a + b; } int main() { return test(1, 4); }", ex_ret: 5 },
            TestData { inst: "int main() { int a; a = 0; do { a = a + 1; } while (a <= 2); return a; }", ex_ret: 3 },
            TestData { inst: "int main() { int i; i = 0; while (1) { i = i + 1; if (i < 100) { continue; } else { break; } } return i; }", ex_ret: 100 },
            TestData { inst: "int main() { int i; i = 0; do { i = i + 1; if (i < 100) { continue; } else { break; } } while(1); return i; }", ex_ret: 100 },
            TestData { inst: "int main() { int i; i = 0; for (;; i = i + 1) { if (i < 100) { continue; } else { break; } } return i; }", ex_ret: 100 },
            TestData { inst: "int main() { return 1; }", ex_ret: 1 },
            TestData { inst: "int main() { return 1 + 2; }", ex_ret: 3 },
            TestData { inst: "int main() { int a; a = 100; return a; }", ex_ret: 100 },
            TestData { inst: "int main() { return 1 == 4; }", ex_ret: 0 },
            TestData { inst: "int main() { int a; a = 0; if (1 == 10) a = 9; else a = 4; return a; }", ex_ret: 4 },
            TestData { inst: "int main() { if (1 != 10) return 1; else return 10; }", ex_ret: 1 },
            TestData { inst: "int main() { int a = 2; a = a * 29; return a; }", ex_ret: 58 },
            TestData { inst: "int main() { int x; x = 7; return *&x; }", ex_ret: 7 },
            TestData { inst: "int main() { int x; int y; x = 7; y = 5; return *&x * *&y; }", ex_ret: 35 },
            TestData { inst: "int main() {\n\tint a = 10;\n    return a;\n }", ex_ret: 10 },
            TestData { inst: "int main() {\n\tint a = 12901;\n    return a == 12901;\n }", ex_ret: 1 },
            TestData { inst: "int main() {\n\tint a = 9;\n\tint *b; b = &a; return *b;\n }", ex_ret: 9 },
            TestData { inst: "int main() {\n\tint a = 9;\n\tint *b = &a; return 10 * *b;\n }", ex_ret: 90 },
        ]
        .iter().enumerate().for_each(|(i, d)| assert_eq!(d.ex_ret, eval(&d.inst.to_string()), "Fail Test: No.{}, inst: {}", i, d.inst));

        // ファイル削除
        let _ = fs::remove_file("test.s");
        let _ = fs::remove_file("test");
    }
}