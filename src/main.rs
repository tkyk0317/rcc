mod arch;
mod asm;
mod ast;
mod config;
mod lexer;
mod map;
mod semantic;
mod symbol;
mod token;

use asm::Asm;
use ast::AstGen;
use lexer::LexicalAnalysis;
use semantic::Semantic;
use std::env;
use std::fs::File;
use std::io::Read;

/// コンパイルスタート
///
/// 成功時、アセンブリを返す。失敗時はエラーのVecを返す
fn compile(inst: &str) -> Result<String, Vec<String>> {
    // 字句解析
    let mut p = LexicalAnalysis::new("stdin".to_string(), &inst);
    p.read_token();

    // AST作成
    let mut ast_gen = AstGen::new(p.get_tokens());
    let ast_tree = ast_gen.parse();

    // 意味解析
    let mut sem = Semantic::new(&ast_tree);
    sem.exec()?;
    let global = sem.get_global_symbol();
    let vars = sem.get_var_symbol();
    let funcs = sem.get_func_symbol();

    // アセンブラへ変換.
    let mut asm = Asm::new(&global, &vars, &funcs);
    asm.exec(&ast_tree);
    Ok(asm.get_inst())
}

#[doc = "メイン関数"]
fn main() {
    // コマンドライン引数評価
    let args: Vec<String> = env::args().collect();

    // 引数チェック
    if args.len() < 2 {
        panic!("Usage: rcc [--input] [filename]")
    }

    // 入力ソースを決定
    let mut s = String::new();
    match &*args[1] {
        "--input" => {
            std::io::stdin().read_line(&mut s).unwrap();
        }
        _ => {
            let mut f = File::open(&args[1]).expect(&format!("not found file {}", args[1]));
            f.read_to_string(&mut s).expect("read file error");
        }
    };

    // コンパイル実行
    match compile(&s) {
        Ok(inst) => println!("{}", inst),
        Err(errs) => errs.iter().for_each(|e| println!("{:?}", e)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::process::Command;

    // テスト用データ構造体
    struct TestData<'a> {
        inst: &'a str,
        ex_ret: i32,
    }

    // アセンブラ書き込み
    fn create_asm_file(inst: &str) -> Result<(), Box<std::error::Error>> {
        let mut file = fs::File::create("test.s")?;
        file.write_all(inst.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    // 評価関数.
    //
    // 引数で指定された文字列をコンパイル→実行、exitコードを返す
    fn eval(inst: &str) -> i32 {
        match compile(inst) {
            Err(_) => -1,
            Ok(inst) => {
                // gccを使用して実行.
                let _ = create_asm_file(&inst);
                let _ = Command::new("gcc")
                    .args(&["-g3", "-no-pie", "./test.s", "-o", "test"])
                    .output();
                match Command::new("./test").status() {
                    Ok(r) => match r.code() {
                        Some(r) => r,
                        None => panic!("code() is failed"),
                    },
                    Err(e) => panic!(e),
                }
            }
        }
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
            TestData { inst: "int test(int x) { return x + 1; } int main() { return test(1); }", ex_ret: 2 },
            TestData { inst: "int test(int x) { x = x * 2; return x + 10; } int main() { int b; b = 10; return test(b); }", ex_ret: 30 },
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
            TestData { inst: "int test(int x, int y) { return x + y; } int main() { return test(1, 4); }", ex_ret: 5 },
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
            TestData { inst: "ttt main() {\n\treturn 1;\n }", ex_ret: -1 },
            TestData { inst: "in main() {\n\treturn 1;\n }", ex_ret: -1 },
            TestData { inst: "int main() {\n\tint* a; int b = 123; a = &b; *a = 20;\nreturn b;\n }", ex_ret: 20 },
            TestData { inst: "int main() {\n\tint* a; int b = 123; a = &b; int c = 99; *a = c;\nreturn b;\n }", ex_ret: 99 },
            TestData { inst: "int main() {int* a; int b = 123; a = &b; int* c; int d = 89; c = &d; *a = *c; return b;\n }", ex_ret: 89 },
            TestData { inst: "int main() {\n\tint* a; int b = 123; a = &b; a = a + 2; int* c = &b; c = c + 2; \nreturn c == a;\n }", ex_ret: 1 },
            TestData { inst: "int main() {\n\tint* a; int b = 123; a = &b; a = a - 4; int* c = &b; c = c - 4; \nreturn c == a;\n }", ex_ret: 1 },
            TestData { inst: "int main() {\n\tint* a; int b = 123; a = &b; a = a + 2; int* c = &b; \nreturn c != a;\n }", ex_ret: 1 },
            TestData { inst: "int main() {\n\tint* a; int b = 123; a = &b; a = a - 4; int* c = &b; \nreturn c != a;\n }", ex_ret: 1 },
            TestData { inst: "int test(int x) { return x; } int main() { int a = 3; return test(a - 1); }", ex_ret: 2 },
            TestData { inst: "int test(int x) { if (x == 0) return 0; else return 1 + test(x - 1); } int main() { return test(10); }", ex_ret: 10 },
            TestData { inst: "int fib(int x) { if (x == 0 || x == 1) return 1; else return fib(x - 2) + fib(x - 1); } int main() { return fib(2); }", ex_ret: 2 },
            TestData { inst: "int fib(int x) { if (x == 0 || x == 1) return 1; else return fib(x - 2) + fib(x - 1); } int main() { return fib(6); }", ex_ret: 13 },
            TestData { inst: "int test(int x) { if(x == 0) return 1; return 2; } int main() { return test(1); }", ex_ret: 2 },
            TestData { inst: "int test(int x) { if(x == 0) return 1; return 2; } int main() { return test(0); }", ex_ret: 1 },
            TestData { inst: "int main() { int a[2]; return 1; }", ex_ret: 1 },
            TestData { inst: "int main() { int x[5]; int i; int* y; for (i = 0; i < 5; i = i + 1) { y = x + i; *y = i; } return *y; }", ex_ret: 4 },
            TestData { inst: "int main() { int* i; int y = 10; i = &y; return *i + 20; }", ex_ret: 30 },
            TestData { inst: "int main() { int* i; int y = 10; i = &y; int x = *i + 2; return x; }", ex_ret: 12 },
            TestData { inst: "int main() { int* i; int y = 10; i = &y; *i = *i + 20; return *i; }", ex_ret: 30, },
            TestData { inst: "int main() { int* i; int y = 10; i = &y; *i = *i - 2; return *i; }", ex_ret: 8, },
            TestData { inst: "int main() { int* i; int y = 10; i = &y; *i = *i + 100 -10; return *i; }", ex_ret: 100, },
            TestData { inst: "int main() { int a, b; a = 10; b = 7; return a * b; }", ex_ret: 70, },
            TestData { inst: "int main() { int a[10]; int *x = a; *(x + 2) = 100; return *(x + 2); }", ex_ret: 100, },
            TestData { inst: "int main() { int a[10]; a[1] = 121; return a[1] * 2; }", ex_ret: 242, },
            TestData { inst: "int main() { int a[10]; a[9] = 200; return a[9] - 100; }", ex_ret: 100, },
            TestData { inst: "int main() { int a[10]; a[0] = 11; return a[0] + 100; }", ex_ret: 111, },
            TestData { inst: "int main() { int a[10]; a[2] = 10; return a[2]; }", ex_ret: 10, },
            TestData { inst: "int main() { int a[10]; int i; for(i = 0 ; i < 10 ; i++) { a[i] = i; } return a[0] + a[1] + a[2]; }", ex_ret: 3, },
            TestData { inst: "int main() { int a[10]; int i; for(i = 0 ; i < 10 ; i++) { a[i] = i; } return a[8] + a[9]; }", ex_ret: 17, },
            TestData { inst: "int main() { int a[10]; int i; for(i = 0 ; i < 10 ; i++) { a[i] = i * 2; } return a[7]; }", ex_ret: 14, },
            TestData { inst: "int main() { int a[10][10]; a[2][9] = 10; return a[2][9]; }", ex_ret: 10, },
            TestData { inst: "int main() { int a[10][10]; a[2][9] = 10; a[1][7] = 7 return a[2][9] + a[1][7]; }", ex_ret: 17, },
            TestData { inst: "int main() { int a[10][10]; int i; for(i = 0 ; i < 10 ; i++) { a[7][i] = i; } return a[7][9] * a[7][2]; }", ex_ret: 18, },
            TestData { inst: "int main() { int a[10][8][2]; a[9][1][1] = 99; return a[9][1][1]; }", ex_ret: 99, },
            TestData { inst: "int main() { int a[10][8][2]; a[0][0][0] = 100; a[9][0][1] = 99; return a[0][0][0] + a[9][0][1]; }", ex_ret: 199, },
            TestData { inst: "int main() { int a[10][8][2]; a[0][0][0] = 100; a[9][0][1] = 99; int x = 2; return a[0][0][0] + a[9][0][1] + x; }", ex_ret: 201, },
            TestData { inst: "int main() { int a[10][10]; int i; int j; for(i = 0 ; i < 10 ; i++) { for (j = 0 ; j < 10 ; j++) { a[i][j] = j; } } return a[0][0] + a[1][7] + a[4][5] + a[9][3]; }", ex_ret: 15, },
            TestData { inst: "int test(int* x) { *x = 100; return 0; } int main() { int a = 3; int *b; b = &a; test(b); return *b; }", ex_ret: 100 },
            TestData { inst: "int main() { int a; int *b; b = &a; *b = 131; return *b - 100; }", ex_ret: 31, },
            TestData { inst: "int b; int main() { b = 10; return b; }", ex_ret: 10 },
            TestData { inst: "int b; int main() { b = 10; return b + 2; }", ex_ret: 12 },
            TestData { inst: "int b; int main() { b = 10; return b * 2; }", ex_ret: 20 },
            TestData { inst: "int a; int main() { a = 10; int b = a; return b + 3; }", ex_ret: 13 },
            TestData { inst: "int a[10]; int main() { a[0] = 10; return a[0]; }", ex_ret: 10 },
            TestData { inst: "int a[10]; int main() { a[0] = 10; a[9] = 9; return a[0] + a[9]; }", ex_ret: 19 },
            TestData { inst: "int* a; int main() { int b; b = 99; a = &b; return *a; }", ex_ret: 99 },
            TestData { inst: "int a[10]; int main() { int i; for (i = 0 ; i < 10 ; i++) { a[i] = i * 2; } return a[1] + a[4] + a[8]; }", ex_ret: 26 },
            TestData { inst: "int main() { int i = 2; i++; return i; }", ex_ret: 3 },
            TestData { inst: "int main() { int i = 2; i-- return i; }", ex_ret: 1 },
            TestData { inst: "int main() { int i = 2; return i++; }", ex_ret: 2 },
            TestData { inst: "int main() { int i = 2; return i-- }", ex_ret: 2 },
            TestData { inst: "int i; int main() { i = 2; i++; return i; }", ex_ret: 3 },
            TestData { inst: "int i; int main() { i = 2; i-- return i; }", ex_ret: 1 },
            TestData { inst: "int i; int main() { i = 2; return i++; }", ex_ret: 2 },
            TestData { inst: "int i; int main() { i = 2; return i-- }", ex_ret: 2 },
            TestData { inst: "int main() { int a[10]; int *b = a; b++; *b = 100; return a[1]; }", ex_ret: 100 },
            TestData { inst: "int main() { int a[10]; int *b = a; b++; *b = 99; return *b; }", ex_ret: 99 },
            TestData { inst: "int main() { int a[10]; int *b = a; b++; b++; b--; *b = 99; return a[1]; }", ex_ret: 99 },
            TestData { inst: "int main() { int a[10]; int *b = a; b++; b++; b--; *b = 99; return *b; }", ex_ret: 99 },
            TestData { inst: "int i; int main() { i = 2; ++i; return i; }", ex_ret: 3 },
            TestData { inst: "int i; int main() { i = 2; --i return i; }", ex_ret: 1 },
            TestData { inst: "int i; int main() { i = 2; return ++i; }", ex_ret: 3 },
            TestData { inst: "int i; int main() { i = 2; return --i }", ex_ret: 1 },
            TestData { inst: "int main() { int a[10]; int *b = a; ++b; *b = 100; return a[1]; }", ex_ret: 100 },
            TestData { inst: "int main() { int a[10]; int *b = a; ++b; *b = 99; return *b; }", ex_ret: 99 },
            TestData { inst: "int main() { int a[10]; int *b = a; ++b; ++b; --b; *b = 99; return a[1]; }", ex_ret: 99 },
            TestData { inst: "int main() { int a[10]; int *b = a; ++b; ++b; --b; *b = 99; return *b; }", ex_ret: 99 },
            TestData { inst: "int main() { char i; i = 2; return i }", ex_ret: 2 },
            TestData { inst: "int main() { char i[2]; i[0] = 0; i[1] = 19; return i[0] + i[1]; }", ex_ret: 19 },
            TestData { inst: "char i; int main() { i = 20; return i }", ex_ret: 20 },
            TestData { inst: "int main() { char* i; char y = 10; i = &y; return *i + 20; }", ex_ret: 30 },
            TestData { inst: "int main() { char* i; char y = 10; i = &y; char x = *i + 2; return x; }", ex_ret: 12 },
            TestData { inst: "int main() { char* i; char y = 10; i = &y; *i = *i + 20; return *i; }", ex_ret: 30, },
            TestData { inst: "int main() { char* i; char y = 10; i = &y; *i = *i - 2; return *i; }", ex_ret: 8, },
            TestData { inst: "int main() { char* i; char y = 10; i = &y; *i = *i + 100 -10; return *i; }", ex_ret: 100, },
            TestData { inst: "int main() { char a[10]; char *x = a; *(x + 2) = 100; return *(x + 2); }", ex_ret: 100, },
            TestData { inst: "char a[10]; char main() { char i; for (i = 0 ; i < 10 ; i++) { a[i] = i * 2; } return a[1] + a[4] + a[8]; }", ex_ret: 26 },
            TestData { inst: "char main() { char i[10]; char *x = i; *(i + 1) = 77; return i[1]; }", ex_ret: 77 },
            TestData { inst: "int main() { char* i; i = \"test\"; return 1; }", ex_ret: 1, },
            TestData { inst: "int main() { char* a; a = \"test\"; char* b; b = \"bbbb\"; return 9; }", ex_ret: 9, },
        ]
        .iter()
        .enumerate()
        .for_each(|(i, d)| {
            assert_eq!(
                d.ex_ret,
                eval(d.inst),
                "\tFail Test: No.{}, inst: {}",
                i,
                d.inst
            )
        });

        // ファイル削除
        let _ = fs::remove_file("test.s");
        let _ = fs::remove_file("test");
    }
}
