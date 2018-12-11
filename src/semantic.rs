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
            AstType::Statement(ref stmt) => self.analysis_statement(stmt),
            AstType::Return(ref s) => self.analysis_return(s),
            AstType::Variable(ref t, ref n) => self.analysis_variable(t, n),
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
        if let Err(ref mut e) = self.analysis(stmt) {
            errs.append(e);
        }
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
}