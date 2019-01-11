use arch::{x64::X64, Generator};
use ast::{AstTree, AstType, Structure, Type};
use config::Config;
use std::process;
use symbol::{Scope, SymbolTable};

#[doc = "ラベル管理"]
struct Label {
    label_no: usize,
    continue_labels: Vec<usize>,
    break_labels: Vec<usize>,
    return_label: usize,
}

impl Label {
    // コンストラクタ.
    pub fn new() -> Self {
        Label {
            label_no: 0,
            continue_labels: vec![],
            break_labels: vec![],
            return_label: 0,
        }
    }

    // ラベル番号インクリメント.
    pub fn next_label(&mut self) -> usize {
        self.label_no += 1;
        self.label_no
    }

    // returnラベル取得.
    pub fn next_return_label(&mut self) -> usize {
        self.return_label = self.next_label();
        self.return_label
    }
    pub fn get_return_label(&self) -> usize {
        self.return_label
    }

    // continueラベル追加.
    pub fn push_continue(&mut self, no: usize) {
        self.continue_labels.push(no);
    }
    // continueラベルpop.
    pub fn pop_continue(&mut self) -> Option<usize> {
        self.continue_labels.pop()
    }
    // continueラベル削除.
    pub fn remove_continue(&mut self, no: usize) {
        self.continue_labels = self
            .continue_labels
            .iter()
            .cloned()
            .filter(|d| *d != no)
            .collect();
    }
    // breakラベル追加.
    pub fn push_break(&mut self, no: usize) {
        self.break_labels.push(no);
    }
    // breakラベルpop.
    pub fn pop_break(&mut self) -> Option<usize> {
        self.break_labels.pop()
    }
    // breakラベル削除.
    pub fn remove_break(&mut self, no: usize) {
        self.break_labels = self
            .break_labels
            .iter()
            .cloned()
            .filter(|d| *d != no)
            .collect();
    }
}

// 関数引数レジスタ.
const REGS: &'static [&str] = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

#[doc = "アセンブラ生成部"]
pub struct Asm<'a> {
    inst: String,
    global_table: &'a SymbolTable,
    var_table: &'a SymbolTable,
    func_table: &'a SymbolTable,
    label: Label,
}

impl<'a> Asm<'a> {
    // コンストラクタ.
    pub fn new(
        global_table: &'a SymbolTable,
        var_table: &'a SymbolTable,
        func_table: &'a SymbolTable,
    ) -> Asm<'a> {
        Asm {
            inst: "".to_string(),
            global_table: global_table,
            var_table: var_table,
            func_table: func_table,
            label: Label::new(),
        }
    }

    // アセンブラ生成部取得
    fn gen_asm(&self) -> impl Generator {
        X64
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
            AstType::Global(ref a) => self.generate_global(a),
            AstType::FuncDef(ref t, ref _s, ref a, ref b, ref c) => {
                self.generate_funcdef(t, a, b, c)
            }
            AstType::Statement(_) => self.generate_statement(ast),
            AstType::While(ref a, ref b) => self.generate_statement_while(a, b),
            AstType::Do(ref a, ref b) => self.generate_statement_do(a, b),
            AstType::If(ref a, ref b, ref c) => self.generate_statement_if(a, b, c),
            AstType::For(ref a, ref b, ref c, ref d) => self.generate_statement_for(a, b, c, d),
            AstType::Continue() => self.generate_statement_continue(),
            AstType::Break() => self.generate_statement_break(),
            AstType::Return(ref a) => self.generate_statement_return(a),
            AstType::Factor(a) => self.generate_factor(a),
            AstType::LogicalAnd(ref a, ref b) => self.generate_logical_and(a, b),
            AstType::LogicalOr(ref a, ref b) => self.generate_logical_or(a, b),
            AstType::Condition(ref a, ref b, ref c) => self.generate_condition(a, b, c),
            AstType::UnPlus(ref a) => self.generate_unplus(a),
            AstType::UnMinus(ref a) => self.generate_unminus(a),
            AstType::Not(ref a) => self.generate_not(a),
            AstType::BitReverse(ref a) => self.generate_bit_reverse(a),
            AstType::Assign(ref a, ref b) => self.generate_assign(a, b),
            AstType::Variable(ref t, ref a, ref s) => self.generate_variable(t, a, s),
            AstType::FuncCall(ref a, ref b) => self.generate_call_func(a, b),
            AstType::PostInc(ref a) => self.generate_post_inc(a),
            AstType::PostDec(ref a) => self.generate_post_dec(a),
            AstType::Plus(ref a, ref b) => self.generate_plus(a, b),
            AstType::Minus(ref a, ref b) => self.generate_minus(a, b),
            AstType::Multiple(ref a, ref b)
            | AstType::Division(ref a, ref b)
            | AstType::Remainder(ref a, ref b)
            | AstType::Equal(ref a, ref b)
            | AstType::NotEqual(ref a, ref b)
            | AstType::LessThan(ref a, ref b)
            | AstType::GreaterThan(ref a, ref b)
            | AstType::LessThanEqual(ref a, ref b)
            | AstType::GreaterThanEqual(ref a, ref b)
            | AstType::LeftShift(ref a, ref b)
            | AstType::RightShift(ref a, ref b)
            | AstType::BitAnd(ref a, ref b)
            | AstType::BitOr(ref a, ref b)
            | AstType::BitXor(ref a, ref b) => self.generate_operator(ast, a, b),
            AstType::Address(ref a) => self.generate_address(a),
            AstType::Indirect(ref a) => self.generate_indirect(a),
            _ => panic!("{} {}: not support expression", file!(), line!()),
        }
    }

    // グローバル変数定義
    fn generate_global(&mut self, a: &Vec<AstType>) {
        self.inst = format!("{}{}", self.inst, "  .data\n");
        a.iter().for_each(|d| match *d {
            AstType::Variable(_, _, ref a) => {
                self.inst = format!("{}{}:\n", self.inst, a);
                self.inst = format!("{}  .zero 8\n", self.inst);
            }
            _ => panic!("not support ast type: {:?}", d),
        })
    }

    // 関数定義.
    fn generate_funcdef(&mut self, _t: &Type, a: &String, b: &AstType, c: &AstType) {
        // return文のラベルを生成.
        let return_label = self.label.next_return_label();

        self.generate_func_start(a);
        self.generate_func_args(b);
        self.generate_statement(c);
        self.generate_label_inst(return_label);
        self.generate_func_end();
    }

    // statement生成.
    fn generate_statement(&mut self, a: &AstType) {
        // 各AstTypeを処理.
        match *a {
            AstType::Statement(ref s) => s.iter().for_each(|ast| {
                self.generate(ast);
                if ast.is_expr() {
                    self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
                }
            }),
            _ => panic!("{} {}: not support expr", file!(), line!()),
        }
    }

    // 関数開始アセンブラ出力.
    fn generate_func_start(&mut self, a: &String) {
        // スタート部分設定.
        let mut start = if a == "main" {
            format!("  .text\n.global {}\n", self.generate_func_symbol(a))
        } else {
            "  .text\n".to_string()
        };

        let pos = self.var_table.count() * 8 + 8;
        start = format!("{}{}{}:\n", self.inst, start, self.generate_func_symbol(a));
        start = format!(
            "{}{}{}{}",
            start,
            self.gen_asm().push("rbp"),
            self.gen_asm().mov("rsp", "rbp"),
            self.gen_asm().sub_imm(pos, "rsp")
        );
        self.inst = format!("{}", start);
    }

    // 関数終了部分アセンブラ生成
    fn generate_func_end(&mut self) {
        let pos = self.var_table.count() * 8 + 8;
        let end = format!(
            "{}{}{}",
            self.gen_asm().add_imm(pos, "rsp"),
            self.gen_asm().pop("rbp"),
            self.gen_asm().ret()
        );
        self.inst = format!("{}{}", self.inst, end);
    }

    // 関数引数生成.
    fn generate_func_args(&mut self, a: &AstType) {
        // レジスタからスタックへ引数を移動(SPを8バイトずつ移動しながら).
        let st = 8;
        match *a {
            AstType::Argment(ref args) => {
                args.iter().zip(REGS.iter()).fold(st, |p, d| {
                    match d.0 {
                        AstType::Variable(_, s, _) if *s == Structure::Pointer => {
                            self.inst = format!(
                                "{}{}",
                                self.inst,
                                self.gen_asm().mov_dst(&d.1, "rbp", -(p as i64))
                            );
                        }
                        _ => {
                            self.inst = format!(
                                "{}{}{}",
                                self.inst,
                                self.gen_asm().mov(&d.1, "rax"),
                                self.gen_asm().movl_dst("eax", "rbp", -(p as i64))
                            );
                        }
                    };
                    p + 8
                });
            }
            _ => panic!("{} {}: not support expr {:?}", file!(), line!(), a),
        }
    }

    // if statement生成.
    fn generate_statement_if(&mut self, a: &AstType, b: &AstType, c: &Option<AstType>) {
        let label_end = self.label.next_label();

        // 条件式部分生成.
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.generate_cmp_inst(1, "eax"); // 等しい場合は、1に設定されている.

        // elseブロック生成.
        match c {
            Some(e) => {
                // if条件が満たされているとき、ifラベルへ
                let label_if = self.label.next_label();
                self.generate_je_inst(label_if);

                // elseブロック生成.
                // block部はAstType::Statementなので、演算結果に対するスタック操作は行わない.
                self.generate(e);
                self.generate_jmp_inst(label_end);

                // ifブロック部生成.
                // block部はAstType::Statementなので、演算結果に対するスタック操作は行わない.
                self.generate_label_inst(label_if);
                self.generate(b);
                self.generate_jmp_inst(label_end);
            }
            _ => {
                // if条件が満たされていない場合、endラベルへ
                self.generate_jne_inst(label_end);

                // ifブロック部生成.
                // block部はAstType::Statementなので、演算結果に対するスタック操作は行わない.
                self.generate(b);
            }
        }
        // 終端ラベル
        self.generate_label_inst(label_end);
    }

    // while statement生成.
    fn generate_statement_while(&mut self, a: &AstType, b: &AstType) {
        let label_begin = self.label.next_label();
        let label_end = self.label.next_label();

        // continue/breakラベル生成.
        self.label.push_continue(label_begin);
        self.label.push_break(label_end);

        // condition部生成.
        self.generate_label_inst(label_begin);
        self.generate(a);
        // conditionが偽であれば、ブロック終端へジャンプ.
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.generate_cmp_inst(0, "eax");
        self.generate_je_inst(label_end);

        // ブロック部生成.
        // block部はAstType::Statementなので、演算結果に対するスタック操作は行わない.
        self.generate(b);
        self.generate_jmp_inst(label_begin);

        // endラベル.
        self.generate_label_inst(label_end);

        // 生成したcontinue/breakラベルを除去.
        self.label.remove_continue(label_begin);
        self.label.remove_break(label_end);
    }

    // do-while statement生成.
    fn generate_statement_do(&mut self, a: &AstType, b: &AstType) {
        let label_begin = self.label.next_label();
        let label_condition = self.label.next_label();
        let label_end = self.label.next_label();

        // continue/breakラベル生成.
        self.label.push_continue(label_condition);
        self.label.push_break(label_end);

        // ブロック部生成.
        self.generate_label_inst(label_begin);
        self.generate(a);

        // condition部生成.
        self.generate_label_inst(label_condition);
        self.generate(b);
        // conditionが真であれば、ブロック先頭へジャンプ.
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.generate_cmp_inst(0, "eax");

        self.generate_jne_inst(label_begin);
        self.generate_label_inst(label_end);

        // 生成したcontinue/breakラベルを除去.
        self.label.remove_continue(label_condition);
        self.label.remove_break(label_end);
    }

    // for statement生成.
    fn generate_statement_for(
        &mut self,
        a: &Option<AstType>,
        b: &Option<AstType>,
        c: &Option<AstType>,
        d: &AstType,
    ) {
        let label_begin = self.label.next_label();
        let label_continue = self.label.next_label();
        let label_end = self.label.next_label();

        // continue/breakラベル生成.
        self.label.push_continue(label_continue);
        self.label.push_break(label_end);

        // 初期条件.
        if let Some(init) = a {
            self.generate(init);
            self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        }
        self.generate_label_inst(label_begin);

        // 終了条件.
        if let Some(cond) = b {
            self.generate(cond);
            self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
            self.generate_cmp_inst(0, "eax");
            self.generate_je_inst(label_end);
        }

        // ブロック部.
        self.generate(d);
        self.generate_label_inst(label_continue);

        // 変数変化部分生成
        if let Some(end) = c {
            self.generate(end);
            self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        }
        self.generate_jmp_inst(label_begin);
        self.generate_label_inst(label_end);

        // 生成したcontinue/breakラベルを除去.
        self.label.remove_continue(label_continue);
        self.label.remove_break(label_end);
    }

    // continue文生成.
    fn generate_statement_continue(&mut self) {
        let label = self.label.pop_continue();
        let no = label.expect("asm.rs(generate_statement_continue): invalid continue label");
        self.generate_jmp_inst(no);
    }

    // break文生成.
    fn generate_statement_break(&mut self) {
        let label = self.label.pop_break();
        let no = label.expect("asm.rs(generate_statement_break): invalid continue label");
        self.generate_jmp_inst(no);
    }

    // return statement.
    fn generate_statement_return(&mut self, a: &AstType) {
        self.generate(a);
        if a.is_expr() {
            self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        }
        let label_no = self.label.get_return_label();
        self.generate_jmp_inst(label_no);
    }

    // assign indirect
    fn generate_assign_indirect(&mut self, a: &AstType, b: &AstType) {
        self.generate(a);
        self.generate(b);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.inst = format!(
            "{}{}{}",
            self.inst,
            self.gen_asm().pop("rcx"),
            self.gen_asm().mov_dst("rax", "rcx", 0)
        );
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
    }

    // assign variable
    fn generate_assign_variable(&mut self, a: &String, t: &Type, s: &Structure, b: &AstType) {
        let ret = self.var_table.search(a).unwrap_or_else(|| {
            self.global_table
                .search(a)
                .expect("asm.rs(generate_assign_variable): error option value")
        });
        let offset = ret.p as i64 * 8 + 8;
        self.generate(b);
        match *t {
            Type::Int if *s == Structure::Identifier => {
                self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
                if ret.scope == Scope::Global {
                    self.inst = format!("{}{}", self.inst, self.gen_asm().mov_to_glb("eax", a));
                } else {
                    self.inst = format!(
                        "{}{}",
                        self.inst,
                        self.gen_asm().movl_dst("eax", "rbp", -offset)
                    );
                }
                self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
            }
            Type::Int if *s == Structure::Pointer => {
                if ret.scope == Scope::Global {
                    self.inst = format!(
                        "{}{}{}{}",
                        self.inst,
                        self.gen_asm().pop("rax"),
                        self.gen_asm().movq_to_glb("rax", a),
                        self.gen_asm().push("rax")
                    );
                } else {
                    self.inst = format!(
                        "{}{}{}{}",
                        self.inst,
                        self.gen_asm().pop("rax"),
                        self.gen_asm().mov_dst("rax", "rbp", -offset),
                        self.gen_asm().push("rax")
                    );
                }
            }
            _ => panic!("{} {}: not support type {:?}", file!(), line!(), t),
        }
    }

    // assign生成.
    fn generate_assign(&mut self, a: &AstType, b: &AstType) {
        match *a {
            AstType::Variable(ref t, ref s, ref a) => self.generate_assign_variable(a, t, s, b),
            AstType::Indirect(ref a) => self.generate_assign_indirect(a, b),
            _ => self.generate(b),
        }
    }

    // typeごとのVariable生成
    fn generate_variable_with_type(
        &mut self,
        t: &Type,
        s: &Structure,
        v: &String,
        scope: &Scope,
        offset: i64,
    ) {
        match *t {
            Type::Int => match s {
                Structure::Identifier => {
                    match scope {
                        Scope::Global => {
                            self.inst =
                                format!("{}{}", self.inst, self.gen_asm().mov_from_glb("eax", v));
                        }
                        _ => {
                            self.inst = format!(
                                "{}{}",
                                self.inst,
                                self.gen_asm().movl_src("rbp", "eax", -offset)
                            );
                        }
                    };
                    self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
                }
                Structure::Pointer => match scope {
                    Scope::Global => {
                        self.inst = format!(
                            "{}{}{}",
                            self.inst,
                            self.gen_asm().movq_from_glb("rax", v),
                            self.gen_asm().push("rax")
                        );
                    }
                    _ => {
                        self.inst = format!(
                            "{}{}{}",
                            self.inst,
                            self.gen_asm().mov_src("rbp", "rax", -offset),
                            self.gen_asm().push("rax")
                        );
                    }
                },
                Structure::Array(size) => {
                    let num: i64 = size.iter().fold(1, |acc, i| acc * *i as i64);
                    self.inst = format!(
                        "{}{}{}",
                        self.inst,
                        self.gen_asm().lea(num * 8),
                        self.gen_asm().push("rax")
                    );
                }
                _ => {}
            },
            _ => panic!("{} {}: not support type {:?}", file!(), line!(), t),
        }
    }

    // variable生成.
    fn generate_variable(&mut self, t: &Type, s: &Structure, v: &String) {
        let ret = self.var_table.search(v).unwrap_or_else(|| {
            self.global_table
                .search(v)
                .expect("asm.rs(generate_variable): error option value")
        });
        let offset = ret.p as i64 * 8 + 8;
        self.generate_variable_with_type(t, s, v, &ret.scope, offset);
    }

    // 関数コール生成.
    fn generate_call_func(&mut self, a: &AstType, b: &AstType) {
        match *a {
            // 関数名.
            AstType::Variable(_, _, ref n) if self.func_table.search(n).is_some() => {
                match *b {
                    AstType::Argment(ref v) => {
                        // 各引数を評価（スタックに積むので、逆順で積んでいく）.
                        v.into_iter().rev().for_each(|d| self.generate(d));

                        // 関数引数をスタックからレジスタへ.
                        v.iter().zip(REGS.iter()).for_each(|d| match d.0 {
                            AstType::Variable(_, s, _) if *s == Structure::Pointer => {
                                self.inst = format!("{}{}", self.inst, self.gen_asm().pop(&d.1));
                            }
                            _ => {
                                self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
                                self.inst =
                                    format!("{}{}", self.inst, self.gen_asm().mov("rax", &d.1));
                            }
                        });
                    }
                    _ => panic!("{} {}: Not Function Argment", file!(), line!()),
                }

                self.inst = format!(
                    "{}{}",
                    self.inst,
                    self.gen_asm().call(&self.generate_func_symbol(n))
                );
                self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
            }
            _ => panic!("{} {}: Not Exists Function name", file!(), line!()),
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
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().not("eax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
    }

    // Not演算子生成.
    fn generate_not(&mut self, a: &AstType) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.generate_cmp_inst(0, "eax");
        self.inst = format!("{}{}", self.inst, self.gen_asm().set("al"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().movz("al", "eax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
    }

    // マイナス単項演算子生成.
    fn generate_unminus(&mut self, a: &AstType) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().neg("eax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
    }

    // プラス単項演算子生成.
    fn generate_unplus(&mut self, a: &AstType) {
        self.generate(a);
    }

    // 三項演算子生成.
    fn generate_condition(&mut self, a: &AstType, b: &AstType, c: &AstType) {
        let label_false = self.label.next_label();
        let label_end = self.label.next_label();

        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.generate_cmp_inst(0, "eax");
        self.generate_je_inst(label_false);

        self.generate(b);
        self.generate_jmp_inst(label_end);
        self.generate_label_inst(label_false);

        self.generate(c);
        self.generate_label_inst(label_end);
    }

    // &&演算子生成.
    fn generate_logical_and(&mut self, a: &AstType, b: &AstType) {
        let label_false = self.label.next_label();
        let label_end = self.label.next_label();

        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.generate_cmp_inst(0, "eax");
        self.generate_je_inst(label_false);
        self.generate(b);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.generate_cmp_inst(0, "eax");
        self.generate_je_inst(label_false);

        self.inst = format!("{}{}", self.inst, self.gen_asm().movl_imm(1, "eax"));
        self.generate_jmp_inst(label_end);
        self.generate_label_inst(label_false);
        self.inst = format!("{}{}", self.inst, self.gen_asm().movl_imm(0, "eax"));
        self.generate_label_inst(label_end);
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
    }

    // ||演算子生成.
    fn generate_logical_or(&mut self, a: &AstType, b: &AstType) {
        let label_true = self.label.next_label();
        let label_end = self.label.next_label();

        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.generate_cmp_inst(0, "eax");
        self.generate_jne_inst(label_true);
        self.generate(b);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.generate_cmp_inst(0, "eax");
        self.generate_jne_inst(label_true);

        self.inst = format!("{}{}", self.inst, self.gen_asm().movl_imm(0, "eax"));
        self.generate_jmp_inst(label_end);
        self.generate_label_inst(label_true);
        self.inst = format!("{}{}", self.inst, self.gen_asm().movl_imm(1, "eax"));
        self.generate_label_inst(label_end);
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
    }

    // 数値生成.
    fn generate_factor(&mut self, a: i64) {
        // 数値.
        self.inst = format!("{}{}", self.inst, self.gen_asm().sub_imm(8, "rsp"));
        self.inst = format!("{}  movq ${}, (%rsp)\n", self.inst, a);
        self.inst = format!("{}{}", self.inst, self.gen_asm().movq_imm_dst("rsp", a, 0));
    }

    // 左辺値変数アドレス取得
    fn generate_lvalue_address(&mut self, a: &AstType) {
        let (sym, name) = match *a {
            AstType::Variable(_, _, ref s) => {
                let sym = self.var_table.search(s).unwrap_or_else(|| {
                    self.global_table
                        .search(s)
                        .expect("asm.rs(generate_post_inc): error option value")
                });
                (sym, s)
            }
            _ => panic!(format!(
                "asm.rs(generate_lvalue_address): Not Support AstType {:?}",
                a
            )),
        };
        // アドレスをraxレジスタへ転送
        self.inst = match sym.scope {
            Scope::Global => format!("{}{}", self.inst, self.gen_asm().lea_glb(name)),
            _ => format!("{}{}", self.inst, self.gen_asm().lea(sym.p as i64 * 8 + 8)),
        };
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
    }

    // 後置インクリメント
    fn generate_post_inc(&mut self, a: &AstType) {
        self.generate_lvalue_address(a);

        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rcx"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().movl_src("rcx", "eax", 0));
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().add_imm(1, "eax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().movl_dst("eax", "rcx", 0));
    }

    // 後置デクリメント
    fn generate_post_dec(&mut self, a: &AstType) {
        self.generate_lvalue_address(a);

        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rcx"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().movl_src("rcx", "eax", 0));
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().sub_imm(1, "eax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().movl_dst("eax", "rcx", 0));
    }

    // ポインタ同士の加算
    fn generate_plus_with_pointer(&mut self, a: &AstType, b: &AstType) {
        self.generate(a);
        self.generate(b);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().mov_imm("rcx", 8));
        self.inst = format!("{}{}", self.inst, self.gen_asm().mul("rcx"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rcx"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().add("rax", "rcx"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rcx"));
    }

    // variable同士の加算
    fn generate_plus_variable(&mut self, a: &AstType, b: &AstType, s: &Structure) {
        match s {
            Structure::Array(_) => self.generate_plus_with_pointer(a, b),
            _ => {
                self.generate(a);
                self.generate(b);

                // 加算処理
                self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rcx"));
                self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
                self.inst = format!("{}{}", self.inst, self.gen_asm().plus());
                self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
            }
        }
    }

    // 加算
    fn generate_plus(&mut self, a: &AstType, b: &AstType) {
        match (a, b) {
            // ポインタ演算チェック
            (AstType::Variable(ref _t1, ref s1, _), _) if *s1 == Structure::Pointer => {
                self.generate_plus_with_pointer(a, b)
            }
            (AstType::Variable(ref _t1, ref s1, _), _) => self.generate_plus_variable(a, b, s1),
            _ => {
                self.generate(a);
                self.generate(b);

                // 加算処理
                self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rcx"));
                self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
                self.inst = format!("{}{}", self.inst, self.gen_asm().plus());
                self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
            }
        }
    }

    // ポインタ同士の減算
    fn generate_minus_with_pointer(&mut self, a: &AstType, b: &AstType) {
        self.generate(a);
        self.generate(b);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().mov_imm("rcx", 8));
        self.inst = format!("{}{}", self.inst, self.gen_asm().mul("rcx"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rcx"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().sub("rax", "rcx"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rcx"));
    }

    // 減算
    fn generate_minus(&mut self, a: &AstType, b: &AstType) {
        match (a, b) {
            (AstType::Variable(ref _t1, ref s1, _), AstType::Variable(ref t2, _, _))
                if *s1 == Structure::Pointer && *t2 == Type::Int =>
            {
                self.generate_minus_with_pointer(a, b)
            }
            (AstType::Variable(ref _t1, ref s1, _), AstType::Factor(_))
                if *s1 == Structure::Pointer =>
            {
                self.generate_minus_with_pointer(a, b)
            }
            _ => {
                self.generate(a);
                self.generate(b);

                // 減算処理
                self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rcx"));
                self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
                self.inst = format!("{}{}", self.inst, self.gen_asm().minus());
                self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
            }
        }
    }

    // 演算子生成.
    fn generate_operator(&mut self, ast: &AstType, a: &AstType, b: &AstType) {
        self.generate(a);
        self.generate(b);

        // 各演算子評価.
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rcx"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.inst = format!("{}{}", self.inst, self.operator(ast));

        // 演算子に応じて退避するレジスタを変更.
        match *ast {
            AstType::Remainder(_, _) => {
                self.inst = format!("{}{}", self.inst, self.gen_asm().push("rdx"));
            }
            _ => {
                self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
            }
        }
    }

    // アドレス演算子.
    fn generate_address(&mut self, a: &AstType) {
        match *a {
            AstType::Variable(ref _t, ref _s, ref a) => {
                let ret = self
                    .var_table
                    .search(a)
                    .expect("asm.rs(generate_address): error option value");
                let pos = ret.p as i64 * 8 + 8;
                self.inst = format!("{}{}", self.inst, self.gen_asm().lea(pos));
                self.inst = format!("{}{}", self.inst, self.gen_asm().push("rax"));
            }
            _ => panic!("{} {}: Not Support Ast {:?}", file!(), line!(), a),
        }
    }

    // 間接演算子.
    fn generate_indirect(&mut self, a: &AstType) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.gen_asm().pop("rax"));
        self.inst = format!("{}{}", self.inst, self.gen_asm().movq_src("rax", "rcx", 0));
        self.inst = format!("{}{}", self.inst, self.gen_asm().push("rcx"));
    }

    // 演算子アセンブラ生成.
    fn operator(&self, ope: &AstType) -> String {
        match *ope {
            AstType::Multiple(_, _) => self.gen_asm().multiple(),
            AstType::Equal(_, _) => self.gen_asm().equal(),
            AstType::NotEqual(_, _) => self.gen_asm().not_equal(),
            AstType::LessThan(_, _) => self.gen_asm().less_than(),
            AstType::GreaterThan(_, _) => self.gen_asm().greater_than(),
            AstType::LessThanEqual(_, _) => self.gen_asm().less_than_equal(),
            AstType::GreaterThanEqual(_, _) => self.gen_asm().greater_than_equal(),
            AstType::LeftShift(_, _) => self.gen_asm().left_shift(),
            AstType::RightShift(_, _) => self.gen_asm().right_shift(),
            AstType::BitAnd(_, _) => self.gen_asm().bit_and(),
            AstType::BitOr(_, _) => self.gen_asm().bit_or(),
            AstType::BitXor(_, _) => self.gen_asm().bit_xor(),
            AstType::Division(_, _) | AstType::Remainder(_, _) => self.gen_asm().bit_division(),
            _ => process::abort(),
        }
    }

    // ラベル命令.
    fn generate_label_inst(&mut self, no: usize) {
        self.inst = format!("{}{}", self.inst, self.gen_asm().label(no));
    }

    // jmp命令生成.
    fn generate_jmp_inst(&mut self, no: usize) {
        self.inst = format!("{}{}", self.inst, self.gen_asm().jmp(no));
    }

    // je命令生成.
    fn generate_je_inst(&mut self, no: usize) {
        self.inst = format!("{}{}", self.inst, self.gen_asm().je(no));
    }

    // jne命令生成.
    fn generate_jne_inst(&mut self, no: usize) {
        self.inst = format!("{}{}", self.inst, self.gen_asm().jne(no));
    }

    // cmp命令生成.
    fn generate_cmp_inst(&mut self, f: usize, r: &str) {
        self.inst = format!("{}{}", self.inst, self.gen_asm().cmpl(f, r));
    }
}
