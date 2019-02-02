use ast;
#[doc = "シンボルテーブル"]
use map::Map;

#[derive(Clone, Debug, PartialEq)]
pub enum Scope {
    Global,
    Local,
    Func,
}

#[derive(Debug)]
pub struct SymbolTable {
    scope: Scope,
    count: usize,
    map: Map<Meta>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Meta {
    pub scope: Scope,
    pub p: usize,
    pub t: ast::Type,
    pub s: ast::Structure,
}

impl SymbolTable {
    #[doc = "コンストラクタ"]
    pub fn new(s: Scope) -> Self {
        SymbolTable {
            scope: s,
            count: 0,
            map: Map::new(),
        }
    }

    #[doc = "シンボル追加"]
    pub fn push(&mut self, k: String, t: &ast::Type, s: &ast::Structure) {
        self.map.add(
            k,
            Meta {
                scope: self.scope.clone(),
                p: self.count,
                t: t.clone(),
                s: s.clone(),
            },
        );
        // 配列の場合は要素数分、進める
        match s {
            ast::Structure::Array(s) => self.count += s.iter().fold(1, |acc, i| acc * i),
            _ => self.count += 1,
        };
    }

    #[doc = "シンボル検索"]
    pub fn search(&self, k: &String) -> Option<&Meta> {
        self.map.search(k)
    }

    #[doc = "シンボル数取得"]
    pub fn count(&self) -> usize {
        self.count
    }
}
/**
 * シンボルテーブル
 */
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeNew {
    Global,        // グローバル
    Func(String),  // ローカルスコープ
    Block(String), // ブロックスコープ
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Short,
    Long,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Structure {
    Identifier,
    Pointer,
    Array(Vec<usize>),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub scope: ScopeNew, // スコープ
    pub var: String,     // 変数名
    pub t: Type,         // 型
    pub strt: Structure, // 構造
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable2 {
    table: Vec<Symbol>,
}

impl Symbol {
    // コンストラクタ
    #[allow(dead_code)]
    pub fn new(scope: ScopeNew, var: String, t: Type, strt: Structure) -> Self {
        Symbol {
            scope: scope,
            var: var,
            t: t,
            strt: strt,
        }
    }
}

impl SymbolTable2 {
    // コンストラクタ
    #[allow(dead_code)]
    pub fn new() -> Self {
        SymbolTable2 { table: vec![] }
    }

    // シンボル登録
    #[allow(dead_code)]
    pub fn register_sym(&mut self, sym: Symbol) {
        // 同じシンボルがなければ、登録
        match self.search(&sym.scope, &sym.var) {
            None => self.table.push(sym),
            _ => {}
        };
    }

    // シンボルサーチ
    #[allow(dead_code)]
    pub fn search(&self, scope: &ScopeNew, var: &String) -> Option<&Symbol> {
        self.table
            .iter()
            .find(|s| s.scope == *scope && s.var == *var)
    }

    // カウント取得
    #[allow(dead_code)]
    pub fn count_all(&self) -> usize {
        self.table.len()
    }
    #[allow(dead_code)]
    pub fn count(&self, scope: &ScopeNew) -> usize {
        self.table
            .iter()
            .filter(|s| s.scope == *scope)
            .collect::<Vec<_>>()
            .len()
    }

    // 変数トータルサイズ
    #[allow(dead_code)]
    pub fn size(&self, scope: &ScopeNew) -> usize {
        let type_size = |t: &Type| match t {
            Type::Int => 8,
            Type::Char => 1,
            _ => 0,
        };

        // 各要素のサイズを畳み込み
        self.table
            .iter()
            .filter(|s| s.scope == *scope)
            .fold(0, |acc, sym| match sym.strt {
                Structure::Pointer => acc + 8,
                Structure::Identifier => acc + type_size(&sym.t),
                // 配列の場合、要素数を考慮
                Structure::Array(ref items) => {
                    acc + items
                        .iter()
                        .fold(0, |acc2, i| acc2 + (i * type_size(&sym.t)))
                }
                _ => acc,
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        {
            let mut s = SymbolTable::new(Scope::Local);
            s.push(
                "key".to_string(),
                &ast::Type::Int,
                &ast::Structure::Identifier,
            );
            assert_eq!(s.count(), 1);
            assert_eq!(
                s.search(&"key".to_string()),
                Some(&Meta {
                    scope: Scope::Local,
                    p: 0,
                    t: ast::Type::Int,
                    s: ast::Structure::Identifier
                })
            )
        }
        {
            let mut s = SymbolTable::new(Scope::Global);
            s.push(
                "key".to_string(),
                &ast::Type::Int,
                &ast::Structure::Identifier,
            );
            assert_eq!(s.count(), 1);
            assert_eq!(s.search(&"not_exist_key".to_string()), None)
        }
    }

    #[test]
    fn test_register_symbol() {
        {
            let mut table = SymbolTable2::new();
            table.register_sym(Symbol::new(
                ScopeNew::Global,
                "a".to_string(),
                Type::Int,
                Structure::Identifier,
            ));

            // 期待値
            assert_eq!(table.size(&ScopeNew::Global), 8);
            assert_eq!(table.count_all(), 1);
            assert_eq!(table.count(&ScopeNew::Global), 1);
            assert_eq!(
                table.search(&ScopeNew::Global, &"a".to_string()),
                Some(&Symbol::new(
                    ScopeNew::Global,
                    "a".to_string(),
                    Type::Int,
                    Structure::Identifier
                ))
            );
        }
        {
            let mut table = SymbolTable2::new();
            table.register_sym(Symbol::new(
                ScopeNew::Global,
                "a".to_string(),
                Type::Int,
                Structure::Array(vec![10]),
            ));

            // 期待値
            assert_eq!(table.size(&ScopeNew::Global), 80);
            assert_eq!(table.count_all(), 1);
            assert_eq!(table.count(&ScopeNew::Global), 1);
            assert_eq!(
                table.search(&ScopeNew::Global, &"a".to_string()),
                Some(&Symbol::new(
                    ScopeNew::Global,
                    "a".to_string(),
                    Type::Int,
                    Structure::Array(vec![10])
                ))
            );
        }
        {
            let mut table = SymbolTable2::new();
            table.register_sym(Symbol::new(
                ScopeNew::Func("test".to_string()),
                "a".to_string(),
                Type::Char,
                Structure::Pointer,
            ));

            // 期待値
            assert_eq!(table.count_all(), 1);
            assert_eq!(table.size(&ScopeNew::Func("test".to_string())), 8);
            assert_eq!(table.count(&ScopeNew::Func("test".to_string())), 1);
            assert_eq!(
                table.search(&ScopeNew::Func("test".to_string()), &"a".to_string()),
                Some(&Symbol::new(
                    ScopeNew::Func("test".to_string()),
                    "a".to_string(),
                    Type::Char,
                    Structure::Pointer
                ))
            );
        }
        {
            let mut table = SymbolTable2::new();
            table.register_sym(Symbol::new(
                ScopeNew::Func("test".to_string()),
                "a".to_string(),
                Type::Char,
                Structure::Identifier,
            ));
            table.register_sym(Symbol::new(
                ScopeNew::Global,
                "a".to_string(),
                Type::Int,
                Structure::Identifier,
            ));

            // 期待値
            assert_eq!(table.count_all(), 2);
            assert_eq!(table.count(&ScopeNew::Global), 1);
            assert_eq!(table.size(&ScopeNew::Global), 8);
            assert_eq!(table.count(&ScopeNew::Func("test".to_string())), 1);
            assert_eq!(table.size(&ScopeNew::Func("test".to_string())), 1);
            assert_eq!(
                table.search(&ScopeNew::Global, &"a".to_string()),
                Some(&Symbol::new(
                    ScopeNew::Global,
                    "a".to_string(),
                    Type::Int,
                    Structure::Identifier
                ))
            );
            assert_eq!(
                table.search(&ScopeNew::Func("test".to_string()), &"a".to_string()),
                Some(&Symbol::new(
                    ScopeNew::Func("test".to_string()),
                    "a".to_string(),
                    Type::Char,
                    Structure::Identifier
                ))
            );
        }
    }
}
