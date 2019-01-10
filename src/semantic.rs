use ast::{AstTree, AstType, Structure, Type};
use std::result::Result;
use symbol::{Scope, SymbolTable};

#[doc = "意味解析部"]
pub struct Semantic<'a> {
    ast: &'a AstTree,
    vars_table: SymbolTable,
    global_table: SymbolTable,
    funcs_table: SymbolTable,
}

// 解析結果返却マクロ
macro_rules! analyzed {
    ($e: expr) => {
        if $e.is_empty() {
            Ok(())
        } else {
            Err($e)
        }
    };
}

impl<'a> Semantic<'a> {
    pub fn new(ast: &'a AstTree) -> Self {
        Semantic {
            ast: ast,
            vars_table: SymbolTable::new(Scope::Local),
            global_table: SymbolTable::new(Scope::Global),
            funcs_table: SymbolTable::new(Scope::Func),
        }
    }

    // Global用シンボルテーブル作成
    fn make_global_sym_table(&mut self, a: &AstType) {
        match *a {
            AstType::Global(ref stmt) => stmt.iter().for_each(|s| self.make_global_sym_table(s)),
            AstType::Variable(ref t, ref s, ref n) => {
                if self.global_table.search(n).is_none() {
                    self.global_table.push(n.to_string(), t, s);
                }
            }
            _ => {}
        }
    }

    // if用シンボルテーブル作成
    fn make_local_sym_table_for_if(&mut self, a: &AstType, b: &Option<AstType>) {
        self.make_local_sym_table(a);
        match b {
            Some(c) => self.make_local_sym_table(&c),
            _ => {}
        };
    }

    // シンボルテーブル作成
    fn make_local_sym_table(&mut self, a: &AstType) {
        // ASTタイプを解析し、シンボルテーブル作成
        match *a {
            AstType::Statement(ref stmt) => stmt.iter().for_each(|s| self.make_local_sym_table(s)),
            AstType::FuncCall(_, ref a) => self.make_local_sym_table(a),
            AstType::Argment(ref args) => args.iter().for_each(|a| self.make_local_sym_table(a)),
            AstType::FuncDef(ref t, ref s, ref n, ref a, ref stmt) => {
                if self.funcs_table.search(n).is_none() {
                    self.funcs_table.push(n.to_string(), &t, &s);
                }
                self.make_local_sym_table(a);
                self.make_local_sym_table(stmt);
            }
            AstType::Variable(ref t, ref s, ref n) => {
                if self.vars_table.search(n).is_none() && self.global_table.search(n).is_none() {
                    self.vars_table.push(n.to_string(), t, s);
                }
            }
            AstType::While(_, ref b) => self.make_local_sym_table(b),
            AstType::Do(ref a, _) => self.make_local_sym_table(a),
            AstType::If(_, ref a, ref b) => {
                // optionがmatchで参照できないので、関数化
                self.make_local_sym_table_for_if(a, b);
            }
            AstType::For(_, _, _, ref a) => self.make_local_sym_table(a),
            AstType::Plus(ref a, ref b)
            | AstType::Minus(ref a, ref b)
            | AstType::Multiple(ref a, ref b)
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
            | AstType::BitXor(ref a, ref b)
            | AstType::Assign(ref a, ref b)
            | AstType::LogicalAnd(ref a, ref b)
            | AstType::LogicalOr(ref a, ref b) => {
                self.make_local_sym_table(a);
                self.make_local_sym_table(b);
            }
            AstType::Condition(ref a, ref b, ref c) => {
                self.make_local_sym_table(a);
                self.make_local_sym_table(b);
                self.make_local_sym_table(c);
            }
            AstType::Return(ref a)
            | AstType::UnPlus(ref a)
            | AstType::UnMinus(ref a)
            | AstType::Not(ref a)
            | AstType::BitReverse(ref a)
            | AstType::Address(ref a)
            | AstType::Indirect(ref a) => self.make_local_sym_table(a),
            _ => {}
        }
    }

    // グローバルシンボルテーブル取得.
    pub fn get_global_symbol(&self) -> &SymbolTable {
        &self.global_table
    }

    // シンボルテーブル取得.
    pub fn get_var_symbol(&self) -> &SymbolTable {
        &self.vars_table
    }

    // 関数シンボルテーブル取得.
    pub fn get_func_symbol(&self) -> &SymbolTable {
        &self.funcs_table
    }

    // 解析開始
    pub fn exec(&mut self) -> Result<(), Vec<String>> {
        let tree = self.ast.get_tree();
        let errs = tree.iter().fold(Vec::<String>::new(), |mut init, t| {
            match t {
                AstType::Global(_) => self.make_global_sym_table(t),
                _ => {}
            };

            self.make_local_sym_table(t);
            match self.analysis(&t) {
                Err(ref mut r) => {
                    init.append(r);
                    init
                }
                Ok(_) => init,
            }
        });
        analyzed!(errs)
    }

    // 解析
    fn analysis(&self, ast: &AstType) -> Result<(), Vec<String>> {
        match ast {
            AstType::FuncDef(ref t, ref s, ref n, ref a, ref stmt) => {
                self.analysis_funcdef(t, s, n, a, stmt)
            }
            AstType::FuncCall(ref v, ref a) => self.analysis_funccall(v, a),
            AstType::Argment(ref args) => self.analysis_argment(args),
            AstType::Statement(ref stmt) => self.analysis_statement(stmt),
            AstType::Return(ref s) => self.analysis_return(s),
            AstType::Variable(ref t, ref s, ref n) => self.analysis_variable(t, s, n),
            AstType::Plus(ref a, ref b)
            | AstType::Minus(ref a, ref b)
            | AstType::Multiple(ref a, ref b)
            | AstType::Division(ref a, ref b) => self.analysis_arithmetic(a, b),
            _ => Ok(()),
        }
    }

    // 関数定義解析
    fn analysis_funcdef(
        &self,
        t: &Type,
        _s: &Structure,
        _name: &String,
        args: &AstType,
        stmt: &AstType,
    ) -> Result<(), Vec<String>> {
        let mut errs = vec![];
        match t {
            Type::Unknown(n) => errs.push(format!("Cannot found Type: {:?}", n)),
            _ => {}
        }
        if let Err(ref mut e) = self.analysis(args) {
            errs.append(e);
        }
        if let Err(ref mut e) = self.analysis(stmt) {
            errs.append(e);
        }
        analyzed!(errs)
    }

    // 関数コール解析
    fn analysis_funccall(&self, v: &AstType, _a: &AstType) -> Result<(), Vec<String>> {
        let mut errs = vec![];
        match v {
            AstType::Variable(ref _t, ref _s, ref n) => {
                if self.funcs_table.search(n).is_none() {
                    errs.push(format!("Not define function name: {:?}", n));
                }
            }
            _ => errs.push(format!("AstType is not Variable: {:?}", v)),
        }
        analyzed!(errs)
    }

    // 関数引数解析
    fn analysis_argment(&self, args: &Vec<AstType>) -> Result<(), Vec<String>> {
        let errs = args.iter().fold(Vec::<String>::new(), |mut acc, ref a| {
            match self.analysis(a) {
                Ok(_) => acc,
                Err(ref mut e) => {
                    acc.append(e);
                    acc
                }
            }
        });
        analyzed!(errs)
    }

    // statement解析
    fn analysis_statement(&self, stmt: &Vec<AstType>) -> Result<(), Vec<String>> {
        let errs = stmt.iter().fold(Vec::<String>::new(), |mut acc, ref s| {
            match self.analysis(s) {
                Ok(_) => acc,
                Err(ref mut e) => {
                    acc.append(e);
                    acc
                }
            }
        });
        analyzed!(errs)
    }

    // return文解析
    fn analysis_return(&self, s: &AstType) -> Result<(), Vec<String>> {
        self.analysis(s)
    }

    // 四則演算解析
    fn analysis_arithmetic(&self, a: &AstType, b: &AstType) -> Result<(), Vec<String>> {
        // 左辺、右辺の型チェック
        let check_both_side = |l: &AstType, r: &AstType| match (l, r) {
            (AstType::Variable(ref t1, _, _), AstType::Variable(ref t2, _, _)) => {
                if t1 != t2 {
                    Some(format!("Both Type Difference: {:?} {:?}", t1, t2))
                } else {
                    None
                }
            }
            _ => None,
        };

        // 左辺、右辺の解析
        let mut errs = vec![];
        if let Err(ref mut e) = self.analysis(a) {
            errs.append(e);
        }
        if let Err(ref mut e) = self.analysis(b) {
            errs.append(e);
        }

        // 左辺、右辺の型解析
        if let Some(e) = check_both_side(a, b) {
            errs.push(e);
        }
        analyzed!(errs)
    }

    // 変数定義解析
    fn analysis_variable(&self, t: &Type, _s: &Structure, _n: &String) -> Result<(), Vec<String>> {
        let mut errs = vec![];
        match t {
            Type::Unknown(n) => errs.push(format!("Cannot found Type: {:?}", n)),
            _ => {}
        }
        analyzed!(errs)
    }
}

#[test]
fn test_func_type() {
    // 正常系
    {
        let ast = vec![AstType::FuncDef(
            Type::Int,
            Structure::Identifier,
            "main".to_string(),
            Box::new(AstType::Argment(vec![])),
            Box::new(AstType::Statement(vec![AstType::Return(Box::new(
                AstType::Variable(Type::Int, Structure::Identifier, "a".to_string()),
            ))])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_ok());
    }
    // 型がおかしい
    {
        let ast = vec![AstType::FuncDef(
            Type::Unknown("aaaa".to_string()),
            Structure::Identifier,
            "main".to_string(),
            Box::new(AstType::Argment(vec![])),
            Box::new(AstType::Statement(vec![AstType::Return(Box::new(
                AstType::Variable(Type::Int, Structure::Identifier, "a".to_string()),
            ))])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 1);
    }
}

#[test]
fn test_variable() {
    // Typeがおかしい
    {
        let ast = vec![AstType::FuncDef(
            Type::Int,
            Structure::Identifier,
            "main".to_string(),
            Box::new(AstType::Argment(vec![])),
            Box::new(AstType::Statement(vec![AstType::Return(Box::new(
                AstType::Variable(
                    Type::Unknown("aaaa".to_string()),
                    Structure::Identifier,
                    "a".to_string(),
                ),
            ))])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 1);
    }
}

#[test]
fn test_arithmetic() {
    // 両辺の型がおかしい
    {
        let ast = vec![AstType::FuncDef(
            Type::Int,
            Structure::Identifier,
            "main".to_string(),
            Box::new(AstType::Argment(vec![])),
            Box::new(AstType::Statement(vec![
                AstType::Plus(
                    Box::new(AstType::Variable(
                        Type::Int,
                        Structure::Identifier,
                        "a1".to_string(),
                    )),
                    Box::new(AstType::Variable(
                        Type::Unknown("aaaa".to_string()),
                        Structure::Identifier,
                        "a2".to_string(),
                    )),
                ),
                AstType::Return(Box::new(AstType::Variable(
                    Type::Int,
                    Structure::Identifier,
                    "r".to_string(),
                ))),
            ])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
    {
        let ast = vec![AstType::FuncDef(
            Type::Int,
            Structure::Identifier,
            "main".to_string(),
            Box::new(AstType::Argment(vec![])),
            Box::new(AstType::Statement(vec![
                AstType::Multiple(
                    Box::new(AstType::Variable(
                        Type::Int,
                        Structure::Identifier,
                        "a1".to_string(),
                    )),
                    Box::new(AstType::Variable(
                        Type::Unknown("aaaa".to_string()),
                        Structure::Identifier,
                        "a2".to_string(),
                    )),
                ),
                AstType::Return(Box::new(AstType::Variable(
                    Type::Int,
                    Structure::Identifier,
                    "r".to_string(),
                ))),
            ])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
    {
        let ast = vec![AstType::FuncDef(
            Type::Int,
            Structure::Identifier,
            "main".to_string(),
            Box::new(AstType::Argment(vec![])),
            Box::new(AstType::Statement(vec![
                AstType::Minus(
                    Box::new(AstType::Variable(
                        Type::Int,
                        Structure::Identifier,
                        "a1".to_string(),
                    )),
                    Box::new(AstType::Variable(
                        Type::Unknown("aaaa".to_string()),
                        Structure::Identifier,
                        "a2".to_string(),
                    )),
                ),
                AstType::Return(Box::new(AstType::Variable(
                    Type::Int,
                    Structure::Identifier,
                    "r".to_string(),
                ))),
            ])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
    {
        let ast = vec![AstType::FuncDef(
            Type::Int,
            Structure::Identifier,
            "main".to_string(),
            Box::new(AstType::Argment(vec![])),
            Box::new(AstType::Statement(vec![
                AstType::Division(
                    Box::new(AstType::Variable(
                        Type::Int,
                        Structure::Identifier,
                        "a1".to_string(),
                    )),
                    Box::new(AstType::Variable(
                        Type::Unknown("aaaa".to_string()),
                        Structure::Identifier,
                        "a2".to_string(),
                    )),
                ),
                AstType::Return(Box::new(AstType::Variable(
                    Type::Int,
                    Structure::Identifier,
                    "r".to_string(),
                ))),
            ])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
}

#[test]
fn test_func_argment() {
    // 正常系
    {
        let ast = vec![AstType::FuncDef(
            Type::Int,
            Structure::Identifier,
            "main".to_string(),
            Box::new(AstType::Argment(vec![AstType::Variable(
                Type::Int,
                Structure::Identifier,
                "a".to_string(),
            )])),
            Box::new(AstType::Statement(vec![AstType::Return(Box::new(
                AstType::Variable(Type::Int, Structure::Identifier, "a".to_string()),
            ))])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_ok());
    }
    // 引数の型がおかしい
    {
        let ast = vec![AstType::FuncDef(
            Type::Int,
            Structure::Identifier,
            "main".to_string(),
            Box::new(AstType::Argment(vec![AstType::Variable(
                Type::Unknown("a".to_string()),
                Structure::Identifier,
                "a".to_string(),
            )])),
            Box::new(AstType::Statement(vec![AstType::Return(Box::new(
                AstType::Variable(Type::Int, Structure::Identifier, "a".to_string()),
            ))])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 1);
    }
    {
        let ast = vec![AstType::FuncDef(
            Type::Int,
            Structure::Identifier,
            "main".to_string(),
            Box::new(AstType::Argment(vec![
                AstType::Variable(
                    Type::Unknown("a".to_string()),
                    Structure::Identifier,
                    "a".to_string(),
                ),
                AstType::Variable(
                    Type::Unknown("b".to_string()),
                    Structure::Identifier,
                    "b".to_string(),
                ),
            ])),
            Box::new(AstType::Statement(vec![AstType::Return(Box::new(
                AstType::Variable(Type::Int, Structure::Identifier, "a".to_string()),
            ))])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
}

#[test]
fn test_func_call() {
    // 関数コールのAstがおかしい
    {
        let ast = vec![AstType::FuncCall(
            Box::new(AstType::Factor(2)),
            Box::new(AstType::Argment(vec![])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 1);
    }
    {
        let ast = vec![AstType::FuncCall(
            Box::new(AstType::Variable(
                Type::Int,
                Structure::Identifier,
                "a".to_string(),
            )),
            Box::new(AstType::Argment(vec![])),
        )];
        let tree = AstTree { tree: ast };
        let r = Semantic::new(&tree).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 1);
    }
}
