use std::result::Result;
use ast::AstTree;
use ast::AstType;
use ast::Type;
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
        let errs = self.ast.get_tree().iter().fold(vec![], |mut acc, t| {
            match self.analysis(&t) {
                Err(r) => {
                    acc.push(r);
                    acc
                }
                Ok(_) => acc,
            }
        });
        if errs.is_empty() { Ok(()) } else { Err(errs) }
    }

    // 解析
    fn analysis(&self, ast: &AstType) -> Result<(), String> {
        match ast {
            AstType::FuncDef(ref t, ref name, ref args, ref stmt) => {
                match t {
                    Type::Unknown(n) => Err(format!("Cannot found Type: {:?}", n)),
                    _ => Ok(())
                }
            },
            _ => Ok(())
        }
    }
}

#[test]
fn test_func_type() {
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
        let r = Semantic::new(&tree, &SymbolTable::new(), &SymbolTable::new()).exec();
        assert!(r.is_ok());
    }
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
        let r = Semantic::new(&tree, &SymbolTable::new(), &SymbolTable::new()).exec();
        assert!(r.is_err());
    }
}