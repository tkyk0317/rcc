use std::process;
use ast::Expr;
use config::Config;

#[doc = "アセンブラ生成部"]
pub struct Asm {
    inst: String,
    label_no: u64,
}

impl Asm {
    // コンストラクタ.
    pub fn new() -> Asm {
        // スタート部分設定.
        let main = if Config::is_mac() {
            "_main".to_string()
        } else {
            "main".to_string()
        };
        let mut start = format!(".global {}\n{}:\n", main, main);
        start = format!("{}{}", start, "  push %rbp\n");
        start = format!("{}{}", start, "  mov %rsp, %rbp\n");

        Asm {
            inst: start,
            label_no: 0,
        }
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
            Expr::Factor(a) => self.generate_factor(a),
            Expr::LogicalAnd(ref a, ref b) => self.generate_logical_and(a, b),
            Expr::LogicalOr(ref a, ref b) => self.generate_logical_or(a, b),
            Expr::Condition(ref a, ref b, ref c) => self.generate_condition(a, b, c),
            Expr::UnPlus(ref a) => self.generate_unplus(a),
            Expr::UnMinus(ref a) => self.generate_unminus(a),
            Expr::Not(ref a) => self.generate_not(a),
            Expr::BitReverse(ref a) => self.generate_bit_reverse(a),
            Expr::Block(ref a, ref b) => self.generate_block(a, b),
            Expr::Assign(ref a, ref b) => self.generate_assign(a, b),
            Expr::Variable(_) => self.generate_variable(),
            Expr::Plus(ref a, ref b) |
            Expr::Minus(ref a, ref b) |
            Expr::Multiple(ref a, ref b) |
            Expr::Division(ref a, ref b) |
            Expr::Remainder(ref a, ref b) |
            Expr::Equal(ref a, ref b) |
            Expr::NotEqual(ref a, ref b) |
            Expr::LessThan(ref a, ref b) |
            Expr::GreaterThan(ref a, ref b) |
            Expr::LessThanEqual(ref a, ref b) |
            Expr::GreaterThanEqual(ref a, ref b) |
            Expr::LeftShift(ref a, ref b) |
            Expr::RightShift(ref a, ref b) |
            Expr::BitAnd(ref a, ref b) |
            Expr::BitOr(ref a, ref b) |
            Expr::BitXor(ref a, ref b) => self.generate_operator(ast, a, b),
        }
    }

    // assign生成.
    fn generate_assign(&mut self, _: &Expr, b: &Expr) {
        self.generate(b);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  movl %eax, -4(%rbp)\n", self.inst);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // variable生成.
    fn generate_variable(&mut self) {
        self.inst = format!("{}  movl -4(%rbp), %eax\n", self.inst);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // block生成.
    fn generate_block(&mut self, a: &Expr, b: &Expr) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.generate(b);
    }

    // bit反転演算子生成.
    fn generate_bit_reverse(&mut self, a: &Expr) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  notl %eax\n", self.inst);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // Not演算子生成.
    fn generate_not(&mut self, a: &Expr) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  cmpl $0, %eax\n", self.inst);
        self.inst = format!("{}  sete %al\n", self.inst);
        self.inst = format!("{}  movzbl %al, %eax\n", self.inst);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // マイナス単項演算子生成.
    fn generate_unminus(&mut self, a: &Expr) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  negl %eax\n", self.inst);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // プラス単項演算子生成.
    fn generate_unplus(&mut self, a: &Expr) {
        self.generate(a);
    }

    // 三項演算子生成.
    fn generate_condition(&mut self, a: &Expr, b: &Expr, c: &Expr) {
        let label_false = self.label_no;
        self.label_no = self.label_no + 1;
        let label_end = self.label_no;
        self.label_no = self.label_no + 1;

        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  cmpl $0, %eax\n", self.inst);
        self.inst = format!("{}  je .L{}\n", self.inst, label_false);

        self.generate(b);
        self.inst = format!("{}  jmp .L{}\n", self.inst, label_end);
        self.inst = format!("{}.L{}:\n", self.inst, label_false);

        self.generate(c);
        self.inst = format!("{}.L{}:\n", self.inst, label_end);
    }

    // &&演算子生成.
    fn generate_logical_and(&mut self, a: &Expr, b: &Expr) {
        let label_false = self.label_no;
        self.label_no = self.label_no + 1;
        let label_end = self.label_no;
        self.label_no = self.label_no + 1;

        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  cmpl $0, %eax\n  je .L{}\n", self.inst, label_false);
        self.generate(b);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  cmpl $0, %eax\n  je .L{}\n", self.inst, label_false);

        self.inst = format!("{}{}", self.inst, "  movl $1, %eax\n");
        self.inst = format!("{}  jmp .L{}\n", self.inst, label_end);
        self.inst = format!("{}.L{}:\n", self.inst, label_false);
        self.inst = format!("{}  movl $0, %eax\n", self.inst);
        self.inst = format!("{}.L{}:\n", self.inst, label_end);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // ||演算子生成.
    fn generate_logical_or(&mut self, a: &Expr, b: &Expr) {
        let label_true = self.label_no;
        self.label_no = self.label_no + 1;
        let label_end = self.label_no;
        self.label_no = self.label_no + 1;

        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  cmpl $0, %eax\n  jne .L{}\n", self.inst, label_true);
        self.generate(b);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  cmpl $0, %eax\n  jne .L{}\n", self.inst, label_true);

        self.inst = format!("{}{}", self.inst, "  movl $0, %eax\n");
        self.inst = format!("{}  jmp .L{}\n", self.inst, label_end);
        self.inst = format!("{}.L{}:\n", self.inst, label_true);
        self.inst = format!("{}  movl $1, %eax\n", self.inst);
        self.inst = format!("{}.L{}:\n", self.inst, label_end);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // 数値生成.
    fn generate_factor(&mut self, a: i64) {
        // 数値.
        self.inst = format!("{}{}", self.inst, "  sub $4, %rsp\n");
        self.inst = format!("{}  movl ${}, 0(%rsp)\n", self.inst, a);
    }

    // 演算子生成.
    fn generate_operator(&mut self, ast: &Expr, a: &Expr, b: &Expr) {
        self.generate(a);
        self.generate(b);

        // 各演算子評価.
        self.inst = format!(
            "{}{}{}",
            self.inst,
            self.pop_stack("ecx"),
            self.pop_stack("eax")
        );
        self.inst = format!("{}{}", self.inst, self.operator(ast));

        // 演算子に応じて退避するレジスタを変更.
        match *ast {
            Expr::Remainder(_, _) => self.inst = format!("{}{}", self.inst, self.push_stack("edx")),
            _ => self.inst = format!("{}{}", self.inst, self.push_stack("eax")),
        }
    }

    // スタックポップ.
    fn pop_stack(&self, reg: &str) -> String {
        format!("  movl 0(%rsp), %{}\n  add $4, %rsp\n", reg)
    }

    // プッシュスタック
    fn push_stack(&self, reg: &str) -> String {
        format!("  sub $4, %rsp\n  movl %{}, 0(%rsp)\n", reg)
    }

    // 演算子アセンブラ生成.
    fn operator(&self, ope: &Expr) -> String {
        match *ope {
            Expr::Multiple(_, _) => "  imull %ecx\n".to_string(),
            Expr::Plus(_, _) => "  addl %ecx, %eax\n".to_string(),
            Expr::Minus(_, _) => "  subl %ecx, %eax\n".to_string(),
            Expr::Division(_, _) |
            Expr::Remainder(_, _) => "  movl $0, %edx\n  idivl %ecx\n".to_string(),
            Expr::Equal(_, _) => "  cmpl %ecx, %eax\n  sete %al\n  movzbl %al, %eax\n".to_string(),
            Expr::NotEqual(_, _) => {
                "  cmpl %ecx, %eax\n  setne %al\n  movzbl %al, %eax\n".to_string()
            }
            Expr::LessThan(_, _) => {
                "  cmpl %ecx, %eax\n  setl %al\n  movzbl %al, %eax\n".to_string()
            }
            Expr::GreaterThan(_, _) => {
                "  cmpl %ecx, %eax\n  setg %al\n  movzbl %al, %eax\n".to_string()
            }
            Expr::LessThanEqual(_, _) => {
                "  cmpl %ecx, %eax\n  setle %al\n  movzbl %al, %eax\n".to_string()
            }
            Expr::GreaterThanEqual(_, _) => {
                "  cmpl %ecx, %eax\n  setge %al\n  movzbl %al, %eax\n".to_string()
            }
            Expr::LeftShift(_, _) => "  sall %cl, %eax\n".to_string(),
            Expr::RightShift(_, _) => "  sarl %cl, %eax\n".to_string(),
            Expr::BitAnd(_, _) => "  andl %ecx, %eax\n".to_string(),
            Expr::BitOr(_, _) => "  orl %ecx, %eax\n".to_string(),
            Expr::BitXor(_, _) => "  xorl %ecx, %eax\n".to_string(),
            _ => process::abort(),
        }
    }
}
