use std::process;
use ast::Expr;

/**
 * アセンブラ生成部.
 */
pub struct Asm {
    inst: String,
}

impl Asm {
    // コンストラクタ.
    pub fn new() -> Asm {
        // スタート部分設定.
        let mut start = format!(".global main\n");
        start = format!("{}{}", start, "main:\n");
        start = format!("{}{}", start, "  push %rbp\n");
        start = format!("{}{}", start, "  mov %rsp, %rbp\n");

        Asm { inst: start }
    }

    // アセンブラ取得
    pub fn get_inst(&self) -> String {
        // 終了部分を結合し、返却.
        let mut end = format!("  movl 0(%rsp), %eax\n");
        end = format!("{}{}", end, "  add $4, %rsp\n");
        end = format!("{}{}", end, "  pop %rbp\n");
        end = format!("{}{}", end, "  ret\n");
        format!("{}{}", self.inst, end)
    }

    // アセンブラ生成.
    pub fn generate(&mut self, ast: &Expr) {
        match *ast {
            Expr::Plus(ref a, ref b) |
            Expr::Minus(ref a, ref b) |
            Expr::Multiple(ref a, ref b) => {
                self.generate(a);
                self.generate(b);

                // 各演算子評価.
                self.inst = format!("{}{}", self.inst, "  movl 0(%rsp), %edx\n  add $4, %rsp\n");
                self.inst = format!("{}{}", self.inst, "  movl 0(%rsp), %eax\n  add $4, %rsp\n");
                self.inst = format!("{}{}", self.inst, self.operator(ast));

                // 演算結果をrspへ退避.
                self.inst = format!("{}{}", self.inst, "  sub $4, %rsp\n  movl %eax, 0(%rsp)\n");
            }
            Expr::Factor(a) => {
                // 数値.
                self.inst = format!("{}{}", self.inst, "  sub $4, %rsp\n");
                self.inst = format!("{}  movl ${}, 0(%rsp)\n", self.inst, a);
            }
            _ => panic!("Not Support Expr")
        }
    }

    // 演算子アセンブラ生成.
    fn operator(&self, ope: &Expr) -> String {
        match *ope {
            Expr::Multiple(_, _) => "  imull %edx\n".to_string(),
            Expr::Plus(_, _)     => "  addl %edx, %eax\n".to_string(),
            Expr::Minus(_, _)    => "  subl %edx, %eax\n".to_string(),
            _ => process::abort()
        }
    }
}
