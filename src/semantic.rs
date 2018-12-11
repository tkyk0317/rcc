use std::result::Result;
use ast::{AstTree, AstType, Type};
use symbol::SymbolTable;

#[doc = "意味解析部"]
pub struct Semantic<'a> {
    ast: &'a AstTree,
    vars: &'a SymbolTable,
    funcs: &'a SymbolTable,
}

// 解析結果返却マクロ
macro_rules! analyzed {
    ($e: expr) => { if $e.is_empty() { Ok(()) } else { Err($e) }}
}

impl<'a> Semantic<'a> {
    pub fn new(ast: &'a AstTree, vars: &'a SymbolTable, funcs: &'a SymbolTable) -> Self {
        Semantic {
            ast: ast,
            vars: vars,
            funcs: funcs,
        }
    }

    // 解析開始
    pub fn exec(&self) -> Result<(), Vec<String>> {
        let errs = self.ast.get_tree().iter().fold(Vec::<String>::new(), |mut init, t| {
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
            AstType::FuncDef(ref t, ref n, ref a, ref s) => self.analysis_funcdef(t, n, a, s),
            AstType::Argment(ref args) => self.analysis_argment(args),
            AstType::Statement(ref stmt) => self.analysis_statement(stmt),
            AstType::Return(ref s) => self.analysis_return(s),
            AstType::Variable(ref t, ref n) => self.analysis_variable(t, n),
            AstType::Plus(ref a, ref b)
            | AstType::Minus(ref a, ref b)
            | AstType::Multiple(ref a, ref b)
            | AstType::Division(ref a, ref b) => self.analysis_arithmetic(a, b),
            _ => Ok(())
        }
    }

    // 関数定義解析
    fn analysis_funcdef(&self, t: &Type, name: &String, args: &AstType, stmt: &AstType) -> Result<(), Vec<String>> {
        let mut errs = vec![];
        match t {
            Type::Unknown(n) => errs.push(format!("Cannot found Type: {:?}", n)),
            _ => {},
        }
        if self.funcs.search(name).is_none() {
            errs.push(format!("Cannot found function name: {}", name));
        }
        if let Err(ref mut e) = self.analysis(args) { errs.append(e); }
        if let Err(ref mut e) = self.analysis(stmt) { errs.append(e); }
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
        let check_both_side = |l: &AstType, r: &AstType|
            match (l, r) {
                (AstType::Variable(ref t1, _), AstType::Variable(ref t2, _)) => {
                    if t1 != t2 { Some(format!("Both Type Difference: {:?} {:?}", t1, t2)) }
                    else { None }
                }
                _ => None
            };

        // 左辺、右辺の解析
        let mut errs = vec![];
        if let Err(ref mut e) = self.analysis(a) { errs.append(e); }
        if let Err(ref mut e) = self.analysis(b) { errs.append(e); }

        // 左辺、右辺の型解析
        if let Some(e) = check_both_side(a, b) { errs.push(e); }
        analyzed!(errs)
    }

    // 変数定義解析
    fn analysis_variable(&self, t: &Type, n: &String) -> Result<(), Vec<String>> {
        let mut errs = vec![];
        match t {
            Type::Unknown(n) => errs.push(format!("Cannot found Type: {:?}", n)),
            _ => {},
        }
        if self.vars.search(n).is_none() {
            errs.push(format!("Cannot found variable: {}", n));
        }
        analyzed!(errs)
    }
}

#[test]
fn test_func_type() {
    // 正常系
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "a".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let mut vars = SymbolTable::new();
        vars.push("a".to_string(), &Type::Int);
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_ok());
    }
    // 型がおかしい
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Unknown("aaaa".to_string()),
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "a".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let mut vars = SymbolTable::new();
        vars.push("a".to_string(), &Type::Int);
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 1);
    }
    // 関数が登録されていない
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "a".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let funcs = SymbolTable::new();
        let mut vars = SymbolTable::new();
        vars.push("a".to_string(), &Type::Int);
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 1);
    }
    // 関数が定義されていないかつ、型がおかしい
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Unknown("aaaa".to_string()),
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "a".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let funcs = SymbolTable::new();
        let mut vars = SymbolTable::new();
        vars.push("a".to_string(), &Type::Int);
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
}

#[test]
fn test_variable() {
    // 変数が登録されていない
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "a".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let vars = SymbolTable::new();
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 1);
    }
    // Typeがおかしい
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Unknown("aaaa".to_string()), "a".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let mut vars = SymbolTable::new();
        vars.push("a".to_string(), &Type::Unknown("aaaa".to_string()));
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 1);
    }
}

#[test]
fn test_arithmetic() {
    // 両辺の型がおかしい
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Plus(
                        Box::new(AstType::Variable(Type::Int, "a1".to_string())),
                        Box::new(AstType::Variable(Type::Unknown("aaaa".to_string()), "a2".to_string())),
                    ),
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "r".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let mut vars = SymbolTable::new();
        vars.push("a1".to_string(), &Type::Int);
        vars.push("a2".to_string(), &Type::Unknown("aaaa".to_string()));
        vars.push("r".to_string(), &Type::Int);
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Multiple(
                        Box::new(AstType::Variable(Type::Int, "a1".to_string())),
                        Box::new(AstType::Variable(Type::Unknown("aaaa".to_string()), "a2".to_string())),
                    ),
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "r".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let mut vars = SymbolTable::new();
        vars.push("a1".to_string(), &Type::Int);
        vars.push("a2".to_string(), &Type::Unknown("aaaa".to_string()));
        vars.push("r".to_string(), &Type::Int);
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Minus(
                        Box::new(AstType::Variable(Type::Int, "a1".to_string())),
                        Box::new(AstType::Variable(Type::Unknown("aaaa".to_string()), "a2".to_string())),
                    ),
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "r".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let mut vars = SymbolTable::new();
        vars.push("a1".to_string(), &Type::Int);
        vars.push("a2".to_string(), &Type::Unknown("aaaa".to_string()));
        vars.push("r".to_string(), &Type::Int);
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Division(
                        Box::new(AstType::Variable(Type::Int, "a1".to_string())),
                        Box::new(AstType::Variable(Type::Unknown("aaaa".to_string()), "a2".to_string())),
                    ),
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "r".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let mut vars = SymbolTable::new();
        vars.push("a1".to_string(), &Type::Int);
        vars.push("a2".to_string(), &Type::Unknown("aaaa".to_string()));
        vars.push("r".to_string(), &Type::Int);
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
}

#[test]
fn test_func_argment() {
    // 正常系
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![
                    AstType::Variable(Type::Int, "a".to_string())
                ])),
                Box::new(AstType::Statement(vec![
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "a".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let mut vars = SymbolTable::new();
        vars.push("a".to_string(), &Type::Int);
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_ok());
    }
    // 引数の型がおかしい
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![
                    AstType::Variable(Type::Unknown("a".to_string()), "a".to_string())
                ])),
                Box::new(AstType::Statement(vec![
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "a".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let mut vars = SymbolTable::new();
        vars.push("a".to_string(), &Type::Int);
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 1);
    }
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![
                    AstType::Variable(Type::Unknown("a".to_string()), "a".to_string()),
                    AstType::Variable(Type::Unknown("b".to_string()), "b".to_string()),
                ])),
                Box::new(AstType::Statement(vec![
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "a".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let mut vars = SymbolTable::new();
        vars.push("a".to_string(), &Type::Unknown("a".to_string()));
        vars.push("b".to_string(), &Type::Unknown("b".to_string()));
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
}