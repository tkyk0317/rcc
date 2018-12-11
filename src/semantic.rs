use std::result::Result;
use ast::{AstTree, AstType, Type};
use symbol::SymbolTable;

#[doc = "意味解析部"]
pub struct Semantic<'a> {
    ast: &'a AstTree,
    vars: &'a SymbolTable,
    funcs: &'a SymbolTable,
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
        if errs.is_empty() { Ok(()) } else { Err(errs) }
    }

    // 解析
    fn analysis(&self, ast: &AstType) -> Result<(), Vec<String>> {
        match ast {
            AstType::FuncDef(ref t, ref n, ref a, ref s) => self.analysis_funcdef(t, n, a, s),
            _ => Ok(())
        }
    }

    // 関数定義解析
    fn analysis_funcdef(&self, t: &Type, name: &String, args: &AstType, stmt: &AstType) -> Result<(), Vec<String>> {
        let mut errs = Vec::<String>::new();
        match t {
            Type::Unknown(n) => errs.push(format!("Cannot found Type: {:?}", n)),
            _ => {},
        }
        if self.funcs.search(name).is_none() {
            errs.push(format!("Cannot found function name: {}", name));
        }
        if errs.is_empty() { Ok(())} else { Err(errs) }
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
                        AstType::Variable(Type::Int, "".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let mut funcs = SymbolTable::new();
        funcs.push("main".to_string(), &Type::Int);
        let vars = SymbolTable::new();
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
                        AstType::Variable(Type::Int, "".to_string())
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
    // 関数が登録されていない
    {
        let ast = vec![
            AstType::FuncDef(
                Type::Int,
                "main".to_string(),
                Box::new(AstType::Argment(vec![])),
                Box::new(AstType::Statement(vec![
                    AstType::Return(Box::new(
                        AstType::Variable(Type::Int, "".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let funcs = SymbolTable::new();
        let vars = SymbolTable::new();
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
                        AstType::Variable(Type::Int, "".to_string())
                    ))
                ]))
            )
        ];
        let tree = AstTree { tree: ast };
        let funcs = SymbolTable::new();
        let vars = SymbolTable::new();
        let r = Semantic::new(&tree, &vars, &funcs).exec();
        assert!(r.is_err());
        assert!(r.err().unwrap().len() == 2);
    }
}