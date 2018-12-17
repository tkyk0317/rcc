use arch::{x64::X64, Generator};
use ast::{AstTree, AstType, Type};
use config::Config;
use std::process;
use symbol::SymbolTable;

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
    var_table: &'a SymbolTable,
    func_table: &'a SymbolTable,
    label: Label,
    gen: Box<Generator>,
}

impl<'a> Asm<'a> {
    // コンストラクタ.
    pub fn new(var_table: &'a SymbolTable, func_table: &'a SymbolTable) -> Asm<'a> {
        Asm {
            inst: "".to_string(),
            var_table: var_table,
            func_table: func_table,
            label: Label::new(),
            gen: Box::new(X64::new()),
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
            AstType::FuncDef(ref t, ref a, ref b, ref c) => self.generate_funcdef(t, a, b, c),
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
            AstType::Variable(ref t, ref a) => self.generate_variable(t, a),
            AstType::CallFunc(ref a, ref b) => self.generate_call_func(a, b),
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
                    self.generate_pop_stack("eax");
                }
            }),
            _ => panic!("{} {}: not support expr", file!(), line!()),
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

        let pos = self.var_table.count() * 8;
        start = format!("{}{}{}:\n", self.inst, start, self.generate_func_symbol(a));
        start = format!("{}{}", start, self.gen.push("rbp"));
        start = format!("{}{}", start, self.gen.mov("rsp", "rbp"));
        start = format!("{}{}", start, self.gen.sub_imm(pos, "rsp"));
        self.inst = format!("{}", start);
    }

    // 関数終了部分アセンブラ生成
    fn generate_func_end(&mut self) {
        let pos = self.var_table.count() * 8;
        let mut end = self.gen.add_imm(pos, "rsp");
        end = format!("{}{}", end, self.gen.pop("rbp"));
        end = format!("{}{}", end, self.gen.ret());
        self.inst = format!("{}{}", self.inst, end);
    }

    // 関数引数生成.
    fn generate_func_args(&mut self, a: &AstType) {
        // レジスタからスタックへ引数を移動(SPを8バイトずつ移動しながら).
        let st = 8;
        match *a {
            AstType::Argment(ref args) => {
                args.iter().zip(REGS.iter()).fold(st, |p, d| {
                    self.inst = format!(
                        "{}{}{}",
                        self.inst,
                        self.gen.mov(&d.1, "rax"),
                        self.gen.movl_dst("eax", "rbp", -(p as i64))
                    );
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
        self.generate_pop_stack("eax");
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
        self.generate_pop_stack("eax");
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
        self.generate_pop_stack("eax");
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
            self.generate_pop_stack("eax");
        }
        self.generate_label_inst(label_begin);

        // 終了条件.
        if let Some(cond) = b {
            self.generate(cond);
            self.generate_pop_stack("eax");
            self.generate_cmp_inst(0, "eax");
            self.generate_je_inst(label_end);
        }

        // ブロック部.
        self.generate(d);
        self.generate_label_inst(label_continue);

        // 変数変化部分生成
        if let Some(end) = c {
            self.generate(end);
            self.generate_pop_stack("eax");
        }
        self.generate_jmp_inst(label_begin);
        self.generate_label_inst(label_end);

        // 生成したcontinue/breakラベルを除去.
        self.label.remove_continue(label_continue);
        self.label.remove_break(label_end);
    }

    // continue文生成.
    fn generate_statement_continue(&mut self) {
        let no = self
            .label
            .pop_continue()
            .expect("asm.rs(generate_statement_continue): invalid continue label");
        self.generate_jmp_inst(no);
    }

    // break文生成.
    fn generate_statement_break(&mut self) {
        let no = self
            .label
            .pop_break()
            .expect("asm.rs(generate_statement_break): invalid continue label");
        self.generate_jmp_inst(no);
    }

    // return statement.
    fn generate_statement_return(&mut self, a: &AstType) {
        self.generate(a);
        if a.is_expr() {
            self.generate_pop_stack("eax");
        }
        let label_no = self.label.get_return_label();
        self.generate_jmp_inst(label_no);
    }

    // 右辺代入式
    fn generate_rhs_assign_indirect(&mut self, a: &AstType) {
        match *a {
            AstType::Indirect(ref b) => self.generate_rhs_assign_indirect(b),
            AstType::Variable(ref t, _) => {
                self.generate(a);
                match *t {
                    Type::Int => {
                        self.generate_pop_stack("eax");
                        self.inst = format!("{}{}", self.inst, self.gen.movl_dst("eax", "rcx", 0));
                    }
                    Type::IntPointer => {
                        self.generate_pop_stack("eax");
                        self.inst = format!(
                            "{}{}{}{}",
                            self.inst,
                            self.gen.pop("rax"),
                            self.gen.mov_dst("rax", "rcx", 0),
                            self.gen.push("rax")
                        );
                    }
                    _ => panic!("{} {}: not support type {:?}", file!(), line!(), t),
                };
            }
            AstType::Factor(_) => {
                self.generate(a);
                self.generate_pop_stack("eax");
                self.inst = format!("{}{}", self.inst, self.gen.movl_dst("eax", "rcx", 0));
            }
            _ => panic!("{} {}: not support AstType {:?}", file!(), line!(), a),
        }
    }

    // assign生成.
    fn generate_assign(&mut self, a: &AstType, b: &AstType) {
        match *a {
            AstType::Variable(ref t, ref a) => {
                let ret = self
                    .var_table
                    .search(a)
                    .expect("asm.rs(generate_assign): error option value");
                let offset = ret.p as i64 * 8 + 8;
                self.generate(b);
                match *t {
                    Type::Int => {
                        self.generate_pop_stack("eax");
                        self.inst =
                            format!("{}{}", self.inst, self.gen.movl_dst("eax", "rbp", -offset));
                        self.generate_push_stack("eax");
                    }
                    Type::IntPointer => {
                        self.inst = format!(
                            "{}{}{}{}",
                            self.inst,
                            self.gen.pop("rax"),
                            self.gen.mov_dst("rax", "rbp", -offset),
                            self.gen.push("rax")
                        );
                    }
                    _ => panic!("{} {}: not support type {:?}", file!(), line!(), t),
                }
            }
            AstType::Indirect(ref a) => {
                self.generate(a);
                self.inst = format!("{}{}", self.inst, self.gen.pop("rcx"));
                self.generate_rhs_assign_indirect(b);
                self.generate_push_stack("eax");
            }
            _ => self.generate(b),
        }
    }

    // variable生成.
    fn generate_variable(&mut self, t: &Type, v: &String) {
        let ret = self
            .var_table
            .search(v)
            .expect("asm.rs(generate_variable): error option value");
        let offset = ret.p as i64 * 8 + 8;
        match *t {
            Type::Int => {
                self.inst = format!("{}{}", self.inst, self.gen.movl_src("rbp", "eax", -offset));
                self.generate_push_stack("eax");
            }
            Type::IntPointer => {
                self.inst = format!(
                    "{}{}{}",
                    self.inst,
                    self.gen.mov_src("rbp", "rax", -offset),
                    self.gen.push("rax")
                )
            }
            _ => panic!("{} {}: not support type {:?}", file!(), line!(), t),
        }
    }

    // 関数コール生成.
    fn generate_call_func(&mut self, a: &AstType, b: &AstType) {
        match *a {
            // 関数名.
            AstType::Variable(_, ref n) => {
                match *b {
                    AstType::Argment(ref v) => {
                        // 各引数を評価（スタックに積むので、逆順で積んでいく）.
                        v.into_iter().rev().for_each(|d| self.generate(d));

                        // 関数引数をスタックからレジスタへ.
                        v.iter().zip(REGS.iter()).for_each(|d| {
                            self.generate_pop_stack("eax");
                            self.inst = format!("{}{}", self.inst, self.gen.mov("rax", &d.1));
                        });
                    }
                    _ => panic!("{} {}: Not Function Argment", file!(), line!()),
                }

                self.inst = format!(
                    "{}{}",
                    self.inst,
                    self.gen.call(&self.generate_func_symbol(n))
                );
                self.generate_push_stack("eax");
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
        self.generate_pop_stack("eax");
        self.inst = format!("{}{}", self.inst, self.gen.not("eax"));
        self.generate_push_stack("eax");
    }

    // Not演算子生成.
    fn generate_not(&mut self, a: &AstType) {
        self.generate(a);
        self.generate_pop_stack("eax");
        self.generate_cmp_inst(0, "eax");
        self.inst = format!("{}{}", self.inst, self.gen.set("al"));
        self.inst = format!("{}{}", self.inst, self.gen.movz("al", "eax"));
        self.generate_push_stack("eax");
    }

    // マイナス単項演算子生成.
    fn generate_unminus(&mut self, a: &AstType) {
        self.generate(a);
        self.generate_pop_stack("eax");
        self.inst = format!("{}{}", self.inst, self.gen.neg("eax"));
        self.generate_push_stack("eax");
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
        self.generate_pop_stack("eax");
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
        self.generate_pop_stack("eax");
        self.generate_cmp_inst(0, "eax");
        self.generate_je_inst(label_false);
        self.generate(b);
        self.generate_pop_stack("eax");
        self.generate_cmp_inst(0, "eax");
        self.generate_je_inst(label_false);

        self.inst = format!("{}{}", self.inst, self.gen.movl_imm(1, "eax"));
        self.generate_jmp_inst(label_end);
        self.generate_label_inst(label_false);
        self.inst = format!("{}{}", self.inst, self.gen.movl_imm(0, "eax"));
        self.generate_label_inst(label_end);
        self.generate_push_stack("eax");
    }

    // ||演算子生成.
    fn generate_logical_or(&mut self, a: &AstType, b: &AstType) {
        let label_true = self.label.next_label();
        let label_end = self.label.next_label();

        self.generate(a);
        self.generate_pop_stack("eax");
        self.generate_cmp_inst(0, "eax");
        self.generate_jne_inst(label_true);
        self.generate(b);
        self.generate_pop_stack("eax");
        self.generate_cmp_inst(0, "eax");
        self.generate_jne_inst(label_true);

        self.inst = format!("{}{}", self.inst, self.gen.movl_imm(0, "eax"));
        self.generate_jmp_inst(label_end);
        self.generate_label_inst(label_true);
        self.inst = format!("{}{}", self.inst, self.gen.movl_imm(1, "eax"));
        self.generate_label_inst(label_end);
        self.generate_push_stack("eax");
    }

    // 数値生成.
    fn generate_factor(&mut self, a: i64) {
        // 数値.
        self.inst = format!("{}{}", self.inst, self.gen.sub_imm(8, "rsp"));
        self.inst = format!("{}{}", self.inst, self.gen.movl_imm_dst(a, "rsp", 0));
    }

    // ポインタ同士の加算
    fn generate_plus_with_pointer(&mut self, a: &AstType, b: &AstType) {
        self.generate(a);
        self.generate(b);
        self.generate_pop_stack("eax");
        self.inst = format!("{}{}", self.inst, self.gen.mov_imm("rdx", 4));
        self.inst = format!("{}{}", self.inst, self.gen.mul("rdx"));
        self.inst = format!("{}{}", self.inst, self.gen.pop("rcx"));
        self.inst = format!("{}{}", self.inst, self.gen.add("rax", "rcx"));
        self.inst = format!("{}{}", self.inst, self.gen.push("rcx"));
    }

    // 加算
    fn generate_plus(&mut self, a: &AstType, b: &AstType) {
        match (a, b) {
            // ポインタ演算チェック
            (AstType::Variable(ref t1, _), AstType::Variable(ref t2, _))
                if *t1 == Type::IntPointer && *t2 == Type::Int =>
            {
                self.generate_plus_with_pointer(a, b)
            }
            (AstType::Variable(ref t1, _), AstType::Factor(_)) if *t1 == Type::IntPointer => {
                self.generate_plus_with_pointer(a, b)
            }
            _ => {
                self.generate(a);
                self.generate(b);

                // 加算処理
                self.generate_pop_stack("ecx");
                self.generate_pop_stack("eax");
                self.inst = format!("{}{}", self.inst, self.gen.plus());
                self.generate_push_stack("eax");
            }
        }
    }

    // ポインタ同士の減算
    fn generate_minus_with_pointer(&mut self, a: &AstType, b: &AstType) {
        self.generate(a);
        self.generate(b);
        self.generate_pop_stack("eax");
        self.inst = format!("{}{}", self.inst, self.gen.mov_imm("rdx", 4));
        self.inst = format!("{}{}", self.inst, self.gen.mul("rdx"));
        self.inst = format!("{}{}", self.inst, self.gen.pop("rcx"));
        self.inst = format!("{}{}", self.inst, self.gen.sub("rax", "rcx"));
        self.inst = format!("{}{}", self.inst, self.gen.push("rcx"));
    }

    // 減算
    fn generate_minus(&mut self, a: &AstType, b: &AstType) {
        match (a, b) {
            (AstType::Variable(ref t1, _), AstType::Variable(ref t2, _))
                if *t1 == Type::IntPointer && *t2 == Type::Int =>
            {
                self.generate_minus_with_pointer(a, b)
            }
            (AstType::Variable(ref t1, _), AstType::Factor(_)) if *t1 == Type::IntPointer => {
                self.generate_minus_with_pointer(a, b)
            }
            _ => {
                self.generate(a);
                self.generate(b);

                // 加算処理
                self.generate_pop_stack("ecx");
                self.generate_pop_stack("eax");
                self.inst = format!("{}{}", self.inst, self.gen.minus());
                self.generate_push_stack("eax");
            }
        }
    }

    // 演算子生成.
    fn generate_operator(&mut self, ast: &AstType, a: &AstType, b: &AstType) {
        self.generate(a);
        self.generate(b);

        // 各演算子評価.
        self.generate_pop_stack("ecx");
        self.generate_pop_stack("eax");
        self.inst = format!("{}{}", self.inst, self.operator(ast));

        // 演算子に応じて退避するレジスタを変更.
        match *ast {
            AstType::Remainder(_, _) => self.generate_push_stack("edx"),
            _ => self.generate_push_stack("eax"),
        }
    }

    // アドレス演算子.
    fn generate_address(&mut self, a: &AstType) {
        match *a {
            AstType::Variable(ref _t, ref a) => {
                let ret = self
                    .var_table
                    .search(a)
                    .expect("asm.rs(generate_address): error option value");
                let pos = ret.p * 8 + 8;
                self.inst = format!("{}{}", self.inst, self.gen.lea(pos));
                self.inst = format!("{}{}", self.inst, self.gen.push("rax"));
            }
            _ => panic!("{} {}: Not Support Ast {:?}", file!(), line!(), a),
        }
    }

    // 間接演算子.
    fn generate_indirect(&mut self, a: &AstType) {
        self.generate(a);
        self.inst = format!("{}{}", self.inst, self.gen.pop("rax"));
        self.inst = format!("{}{}", self.inst, self.gen.movl_src("rax", "ecx", 0));
        self.generate_push_stack("ecx");
    }

    // スタックポップ.
    fn generate_pop_stack(&mut self, reg: &str) {
        self.inst = format!("{}{}", self.inst, self.gen.pop_stack(reg));
    }

    // プッシュスタック
    fn generate_push_stack(&mut self, reg: &str) {
        self.inst = format!("{}{}", self.inst, self.gen.push_stack(reg));
    }

    // 演算子アセンブラ生成.
    fn operator(&self, ope: &AstType) -> String {
        match *ope {
            AstType::Multiple(_, _) => self.gen.multiple(),
            AstType::Equal(_, _) => self.gen.equal(),
            AstType::NotEqual(_, _) => self.gen.not_equal(),
            AstType::LessThan(_, _) => self.gen.less_than(),
            AstType::GreaterThan(_, _) => self.gen.greater_than(),
            AstType::LessThanEqual(_, _) => self.gen.less_than_equal(),
            AstType::GreaterThanEqual(_, _) => self.gen.greater_than_equal(),
            AstType::LeftShift(_, _) => self.gen.left_shift(),
            AstType::RightShift(_, _) => self.gen.right_shift(),
            AstType::BitAnd(_, _) => self.gen.bit_and(),
            AstType::BitOr(_, _) => self.gen.bit_or(),
            AstType::BitXor(_, _) => self.gen.bit_xor(),
            AstType::Division(_, _) | AstType::Remainder(_, _) => self.gen.bit_division(),
            _ => process::abort(),
        }
    }

    // ラベル命令.
    fn generate_label_inst(&mut self, no: usize) {
        self.inst = format!("{}{}", self.inst, self.gen.label(no));
    }

    // jmp命令生成.
    fn generate_jmp_inst(&mut self, no: usize) {
        self.inst = format!("{}{}", self.inst, self.gen.jmp(no));
    }

    // je命令生成.
    fn generate_je_inst(&mut self, no: usize) {
        self.inst = format!("{}{}", self.inst, self.gen.je(no));
    }

    // jne命令生成.
    fn generate_jne_inst(&mut self, no: usize) {
        self.inst = format!("{}{}", self.inst, self.gen.jne(no));
    }

    // cmp命令生成.
    fn generate_cmp_inst(&mut self, f: usize, r: &str) {
        self.inst = format!("{}{}", self.inst, self.gen.cmpl(f, r));
    }
}
