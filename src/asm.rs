use std::process;
use ast::AstType;
use ast::AstTree;
use config::Config;
use symbol::SymbolTable;

#[doc = "アセンブラ生成部"]
pub struct Asm<'a> {
    inst: String,
    label_no: u64,
    var_table: &'a SymbolTable,
    func_table: &'a SymbolTable,
}

// 関数引数レジスタ.
const REGS: &'static [&str] = &["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];

impl<'a> Asm<'a> {
    // コンストラクタ.
    pub fn new(var_table: &'a SymbolTable, func_table: &'a SymbolTable) -> Asm<'a> {
        Asm {
            inst: "".to_string(),
            label_no: 0,
            var_table: var_table,
            func_table: func_table,
        }
    }

    // アセンブラ取得
    pub fn get_inst(&self) -> String {
        self.inst.clone()
    }

    // アセンブラ生成開始.
    pub fn exec(&mut self, tree: &AstTree) {
        tree.get_tree().iter().for_each(|a| self.generate(a));
    }

    // アセンブラ生成.
    fn generate(&mut self, ast: &AstType) {
        match *ast {
            AstType::FuncDef(ref a, ref b, ref c) => self.generate_funcdef(a, b, c),
            AstType::Statement(_) => self.generate_statement(ast),
            AstType::While(ref a, ref b) => self.generate_statement_while(a, b),
            AstType::Do(ref a, ref b) => self.generate_statement_do(a, b),
            AstType::If(ref a, ref b, ref c) => self.generate_statement_if(a, b, c),
            AstType::For(ref a, ref b, ref c, ref d) => self.generate_statement_for(a, b, c, d),
            AstType::Factor(a) => self.generate_factor(a),
            AstType::LogicalAnd(ref a, ref b) => self.generate_logical_and(a, b),
            AstType::LogicalOr(ref a, ref b) => self.generate_logical_or(a, b),
            AstType::Condition(ref a, ref b, ref c) => self.generate_condition(a, b, c),
            AstType::UnPlus(ref a) => self.generate_unplus(a),
            AstType::UnMinus(ref a) => self.generate_unminus(a),
            AstType::Not(ref a) => self.generate_not(a),
            AstType::BitReverse(ref a) => self.generate_bit_reverse(a),
            AstType::Assign(ref a, ref b) => self.generate_assign(a, b),
            AstType::Variable(ref a) => self.generate_variable(a),
            AstType::CallFunc(ref a, ref b) => self.generate_call_func(a, b),
            AstType::Plus(ref a, ref b) |
            AstType::Minus(ref a, ref b) |
            AstType::Multiple(ref a, ref b) |
            AstType::Division(ref a, ref b) |
            AstType::Remainder(ref a, ref b) |
            AstType::Equal(ref a, ref b) |
            AstType::NotEqual(ref a, ref b) |
            AstType::LessThan(ref a, ref b) |
            AstType::GreaterThan(ref a, ref b) |
            AstType::LessThanEqual(ref a, ref b) |
            AstType::GreaterThanEqual(ref a, ref b) |
            AstType::LeftShift(ref a, ref b) |
            AstType::RightShift(ref a, ref b) |
            AstType::BitAnd(ref a, ref b) |
            AstType::BitOr(ref a, ref b) |
            AstType::BitXor(ref a, ref b) => self.generate_operator(ast, a, b),
            _ => panic!("asm.rs(generate): not support expression"),
        }
    }

    // 関数定義.
    fn generate_funcdef(&mut self, a: &String, b: &AstType, c: &AstType) {
        self.generate_func_start(a);
        self.generate_func_args(b);
        self.generate_statement(c);
        self.generate_func_end();
    }

    // statement生成.
    fn generate_statement(&mut self, a: &AstType) {
        // アセンブリ生成.
        let mut gen = move |ast| {
            self.generate(ast);
            if ast.is_expr() {
                self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
            }
        };

        // 各AstTypeを処理.
        match *a {
            AstType::Statement(ref s) => s.iter().for_each(|ast| gen(ast)),
            _ => panic!("asm.rs(generate_statement): not support expr"),
        }
    }

    // 関数開始アセンブラ出力.
    fn generate_func_start(&mut self, a: &String) {
        // スタート部分設定.
        let mut start = if a == "main" {
            format!(".global {}\n", self.generate_func_symbol(a))
        } else {
            "".to_string()
        };

        let pos = self.var_table.count() * 4;
        start = format!("{}{}{}:\n", self.inst, start, self.generate_func_symbol(a));
        start = format!("{}{}", start, "  push %rbp\n");
        start = format!("{}{}", start, "  mov %rsp, %rbp\n");
        start = format!("{}  sub ${}, %rsp\n", start, pos);
        self.inst = format!("{}", start);
    }

    // 関数終了部分アセンブラ生成
    fn generate_func_end(&mut self) {
        let pos = self.var_table.count() * 4;
        let mut end = format!("  add ${}, %rsp\n", pos);
        end = format!("{}{}", end, "  pop %rbp\n");
        end = format!("{}{}", end, "  ret\n");
        self.inst = format!("{}{}", self.inst, end);
    }

    // 関数引数生成.
    fn generate_func_args(&mut self, a: &AstType) {
        // 各引数生成.
        let gen = |inst: &str, a: &AstType, r: &str, p: usize| -> String {
            match a {
                AstType::Variable(_) => {
                    let mut t = format!("{}  mov {}, %rax\n", inst, r);
                    format!("{}  movl %eax, -{}(%rbp)\n", t, p)
                }
                _ => panic!("asm.rs(generate_each_args): not variable {:?}", a),
            }
        };

        // レジスタからスタックへ引数を移動(SPを4バイトずつ移動しながら).
        let st = 4;
        match *a {
            AstType::Argment(ref args) => {
                args.iter()
                    .zip(REGS.iter())
                    .fold(st, |p, d| {
                        self.inst = gen(&self.inst, d.0, d.1, p);
                        p + 4
                    });
            }
            _ => panic!("asm.rs(generate_func_args): not support expr {:?}", a),
        }
    }

    // if statement生成.
    fn generate_statement_if(&mut self, a: &AstType, b: &AstType, c: &Option<AstType>) {
        self.label_no = self.label_no + 1;
        let label_end = self.label_no;

        // 条件式部分生成.
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  cmpl $1, %eax\n", self.inst); // 等しい場合は、1に設定されている.
        self.inst = format!("{}  je .L{}\n", self.inst, label_end);

        // elseブロック生成.
        if let Some(e) = c {
            self.label_no = self.label_no + 1;
            let label_else = self.label_no;

            // elseブロック生成.
            // block部はAstType::Statementなので、演算結果に対するスタック操作は行わない.
            self.generate(e);
            self.inst = format!("{}  jmp .L{}\n", self.inst, label_else);

            // ifブロック部生成.
            // block部はAstType::Statementなので、演算結果に対するスタック操作は行わない.
            self.inst = format!("{}.L{}:\n", self.inst, label_end);
            self.generate(b);

            // 終端ラベル.
            self.inst = format!("{}.L{}:\n", self.inst, label_else);
        }
        else {
            // ifブロック部生成.
            // block部はAstType::Statementなので、演算結果に対するスタック操作は行わない.
            self.inst = format!("{}.L{}:\n", self.inst, label_end);
            self.generate(b);
        }
    }

    // while statement生成.
    fn generate_statement_while(&mut self, a: &AstType, b: &AstType) {
        let label_begin = self.label_no + 1;
        self.label_no = label_begin;
        let label_end = self.label_no + 1;
        self.label_no = label_end;

        // condition部生成.
        self.inst = format!("{}.L{}:\n", self.inst, label_begin);
        self.generate(a);
        // conditionが偽であれば、ブロック終端へジャンプ.
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  cmpl $0, %eax\n", self.inst);
        self.inst = format!("{}  je .L{}\n", self.inst, label_end);

        // ブロック部生成.
        // block部はAstType::Statementなので、演算結果に対するスタック操作は行わない.
        self.generate(b);
        self.inst = format!("{}  jmp .L{}\n", self.inst, label_begin);

        // endラベル.
        self.inst = format!("{}.L{}:\n", self.inst, label_end);
    }

    // do-while statement生成.
    fn generate_statement_do(&mut self, a: &AstType, b: &AstType) {
        let label_begin = self.label_no + 1;
        self.label_no = label_begin;
        let label_end = self.label_no + 1;
        self.label_no = label_end;

        // ブロック部生成.
        self.inst = format!("{}.L{}:\n", self.inst, label_begin);
        self.generate(a);

        // condition部生成.
        self.generate(b);
        // conditionが真であれば、ブロック先頭へジャンプ.
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  cmpl $0, %eax\n", self.inst);
        self.inst = format!("{}  jne .L{}\n", self.inst, label_begin);
        self.inst = format!("{}.L{}:\n", self.inst, label_end);
    }

    // for statement生成.
    fn generate_statement_for(&mut self, a: &Option<AstType>, b: &Option<AstType>, c: &Option<AstType>, d: &AstType) {
        self.label_no = self.label_no + 1;
        let label_begin = self.label_no;
        self.label_no = self.label_no + 1;
        let label_end = self.label_no;

        // 初期条件.
        if let Some(init) = a {
            self.generate(init);
            self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        }
        self.inst = format!("{}.L{}:\n", self.inst, label_begin);

        // 終了条件.
        if let Some(cond) = b {
            self.generate(cond);
            self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
            self.inst = format!("{}  cmpl $0, %eax\n", self.inst);
            self.inst = format!("{}  je .L{}\n", self.inst, label_end);
        }

        // ブロック部.
        self.generate(d);

        // 変数変化部分生成
        if let Some(end) = c {
            self.generate(end);
            self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        }
        self.inst = format!("{}  jmp .L{}\n", self.inst, label_begin);
        self.inst = format!("{}.L{}:\n", self.inst, label_end);
    }

    // assign生成.
    fn generate_assign(&mut self, a: &AstType, b: &AstType) {
        match *a {
            AstType::Variable(ref a) => {
                let pos = self.var_table.search(a).unwrap().pos * 4 + 4;
                self.generate(b);
                self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
                self.inst = format!("{}  movl %eax, -{}(%rbp)\n", self.inst, pos);
                self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
            }
            _ => self.generate(b),
        }
    }

    // variable生成.
    fn generate_variable(&mut self, v: &String) {
        let pos = self.var_table.search(v).unwrap().pos * 4 + 4;
        self.inst = format!("{}  movl -{}(%rbp), %eax\n", self.inst, pos);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // 関数コール生成.
    fn generate_call_func(&mut self, a: &AstType, b: &AstType) {
        match *a {
            // 関数名.
            AstType::Variable(ref n) => {
                match *b {
                    AstType::Argment(ref v) => {
                        // 各引数を評価（スタックに積むので、逆順で積んでいく）.
                        v.into_iter().rev().for_each(|d| self.generate(d));

                        // 関数引数をスタックからレジスタへ.
                        self.inst = v.iter()
                                     .zip(REGS.iter())
                                     .fold(self.inst.clone(), |s, d| s + &self.pop_stack("eax") + &format!("  mov %rax, {}\n", d.1));
                    }
                    _ => panic!("asm.rs(generate_call_func): Not Function Argment"),
                }

                self.inst = format!("{}  call {}\n", self.inst, self.generate_func_symbol(n));
                self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
            }
            _ => panic!("asm.rs(generate_call_func): Not Exists Function name"),
        }
    }

    // 関数シンボル生成.
    fn generate_func_symbol(&self, s: &String) -> String {
        if Config::is_mac() {
            format!("_{}", s)
        } else {
            s.to_string()
        }
    }

    // bit反転演算子生成.
    fn generate_bit_reverse(&mut self, a: &AstType) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  notl %eax\n", self.inst);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // Not演算子生成.
    fn generate_not(&mut self, a: &AstType) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  cmpl $0, %eax\n", self.inst);
        self.inst = format!("{}  sete %al\n", self.inst);
        self.inst = format!("{}  movzbl %al, %eax\n", self.inst);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // マイナス単項演算子生成.
    fn generate_unminus(&mut self, a: &AstType) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.pop_stack("eax"));
        self.inst = format!("{}  negl %eax\n", self.inst);
        self.inst = format!("{}{}", self.inst, self.push_stack("eax"));
    }

    // プラス単項演算子生成.
    fn generate_unplus(&mut self, a: &AstType) {
        self.generate(a);
    }

    // 三項演算子生成.
    fn generate_condition(&mut self, a: &AstType, b: &AstType, c: &AstType) {
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
    fn generate_logical_and(&mut self, a: &AstType, b: &AstType) {
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
    fn generate_logical_or(&mut self, a: &AstType, b: &AstType) {
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
    fn generate_operator(&mut self, ast: &AstType, a: &AstType, b: &AstType) {
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
            AstType::Remainder(_, _) => self.inst = format!("{}{}", self.inst, self.push_stack("edx")),
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
    fn operator(&self, ope: &AstType) -> String {
        match *ope {
            AstType::Multiple(_, _) => "  imull %ecx\n".to_string(),
            AstType::Plus(_, _) => "  addl %ecx, %eax\n".to_string(),
            AstType::Minus(_, _) => "  subl %ecx, %eax\n".to_string(),
            AstType::Equal(_, _) => "  cmpl %ecx, %eax\n  sete %al\n  movzbl %al, %eax\n".to_string(),
            AstType::NotEqual(_, _) => "  cmpl %ecx, %eax\n  setne %al\n  movzbl %al, %eax\n".to_string(),
            AstType::LessThan(_, _) => "  cmpl %ecx, %eax\n  setl %al\n  movzbl %al, %eax\n".to_string(),
            AstType::GreaterThan(_, _) => "  cmpl %ecx, %eax\n  setg %al\n  movzbl %al, %eax\n".to_string(),
            AstType::LessThanEqual(_, _) => "  cmpl %ecx, %eax\n  setle %al\n  movzbl %al, %eax\n".to_string(),
            AstType::GreaterThanEqual(_, _) => "  cmpl %ecx, %eax\n  setge %al\n  movzbl %al, %eax\n".to_string(),
            AstType::LeftShift(_, _) => "  sall %cl, %eax\n".to_string(),
            AstType::RightShift(_, _) => "  sarl %cl, %eax\n".to_string(),
            AstType::BitAnd(_, _) => "  andl %ecx, %eax\n".to_string(),
            AstType::BitOr(_, _) => "  orl %ecx, %eax\n".to_string(),
            AstType::BitXor(_, _) => "  xorl %ecx, %eax\n".to_string(),
            AstType::Division(_, _) |
            AstType::Remainder(_, _) => "  movl $0, %edx\n  idivl %ecx\n".to_string(),
            _ => process::abort(),
        }
    }
}
