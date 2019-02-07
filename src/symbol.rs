/**
 * シンボルテーブル
 */
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    Global,         // グローバル
    Local(String),  // ローカルスコープ
    Block(String),  // ブロックスコープ
    Func,           // 関数シンボル
    Unknown,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Short,
    Long,
    Unknown(String),
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
    pub scope: Scope,    // スコープ
    pub var: String,     // 変数名
    pub t: Type,         // 型
    pub strt: Structure, // 構造
    pub pos: usize,      // ポジション
    pub offset: usize,   // オフセット
    pub size: usize,     // サイズ
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    table: Vec<Symbol>,
}

impl Symbol {
    // コンストラクタ
    #[allow(dead_code)]
    pub fn new(scope: Scope, var: String, t: Type, strt: Structure) -> Self {
        Symbol {
            scope: scope,
            var: var,
            t: t,
            strt: strt,
            pos: 0,
            offset: 0,
            size: 0,
        }
    }
}

impl SymbolTable {
    // コンストラクタ
    #[allow(dead_code)]
    pub fn new() -> Self {
        SymbolTable { table: vec![] }
    }

    // シンボル登録
    #[allow(dead_code)]
    pub fn register_sym(&mut self, sym: Symbol) {
        // 同じシンボルがなければ、登録
        if self.search(&sym.scope, &sym.var).is_none() {
            match sym.scope {
                Scope::Func => self.register_func(sym),
                _ => self.register_variable(sym),
            }
        }
    }

    // 関数シンボル登録
    fn register_func(&mut self, sym: Symbol) {
        // 関数シンボルの場合、ポジション算出は不要なのでそのまま登録
        let mut reg = sym.clone();
        reg.pos = 1;
        reg.offset = 0;
        reg.size = 8; // 関数ポインタサイズとして登録
        self.table.push(reg);
    }

    // 変数シンボル登録
    fn register_variable(&mut self, sym: Symbol) {
        // 同じスコープの最終要素からポジションを決定
        let mut reg = sym.clone();
        let last = self
            .table
            .iter()
            .filter(|s| s.scope == sym.scope)
            .cloned()
            .last();

        // 前の要素をもとにオフセット等の情報を算出
        match last {
            None => {
                // 配列の場合、要素数を考慮し、サイズ算出
                let size = match sym.strt {
                    Structure::Array(ref v) => self.type_size(&sym.t, &sym.strt) * v.iter().fold(1, |acc, item| acc * item),
                    _ => self.type_size(&sym.t, &sym.strt)
                };
                reg.pos = 1;
                reg.offset = 0;
                reg.size = size;
                self.table.push(reg);
            }
            Some(pre_sym) => {
                // 配列の場合、要素数を考慮
                match pre_sym.strt {
                    Structure::Array(ref v) => {
                        // 要素数分、オフセットなどを計算
                        let count = v.iter().fold(1, |acc, item| acc * item);
                        reg.pos = pre_sym.pos + count;
                        reg.size = self.type_size(&sym.t, &sym.strt) * count;
                        reg.offset = pre_sym.offset + self.type_size(&pre_sym.t, &pre_sym.strt) * count;
                        reg.offset = (reg.offset / 8) * 8;
                        self.table.push(reg);
                    }
                    _ => {
                        reg.pos = pre_sym.pos + 1;
                        reg.size = self.type_size(&sym.t, &sym.strt);
                        reg.offset = pre_sym.offset + self.type_size(&pre_sym.t, &pre_sym.strt);
                        reg.offset = (reg.offset / 8) * 8;
                        self.table.push(reg);
                    }
                }
            }
        };
    }

    // シンボルサーチ
    #[allow(dead_code)]
    pub fn search(&self, scope: &Scope, var: &String) -> Option<Symbol> {
        self.table
            .iter()
            .find(|s| s.scope == *scope && s.var == *var)
            .cloned()
    }

    // カウント取得
    #[allow(dead_code)]
    pub fn count_all(&self) -> usize {
        self.table.len()
    }
    #[allow(dead_code)]
    pub fn count(&self, scope: &Scope) -> usize {
        self.table
            .iter()
            .filter(|s| s.scope == *scope)
            .collect::<Vec<_>>()
            .len()
    }

    // 型に応じたサイズ取得
    fn type_size(&self, t: &Type, s: &Structure) -> usize {
        match s {
            Structure::Pointer => 8,
            Structure::Array(_) => 8,
            _ => {
                match t {
                    Type::Int => 8,
                    Type::Char => 1,
                    _ => 0,
                }
            }
        }
    }

    // 変数トータルサイズ
    #[allow(dead_code)]
    pub fn size(&self, scope: &Scope) -> usize {
        // 各要素のサイズを畳み込み
        self.table
            .iter()
            .filter(|s| s.scope == *scope)
            .fold(0, |acc, sym| match sym.strt {
                Structure::Pointer => acc + 8,
                Structure::Identifier => acc + self.type_size(&sym.t, &sym.strt),
                // 配列の場合、要素数を考慮
                Structure::Array(ref items) => {
                    acc + items.iter().fold(0, |acc2, i| acc2 + (i * self.type_size(&sym.t, &sym.strt)))
                }
                _ => acc,
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register_symbol() {
        {
            let mut table = SymbolTable::new();
            table.register_sym(Symbol::new(
                Scope::Global,
                "a".to_string(),
                Type::Int,
                Structure::Identifier,
            ));

            // 期待値
            assert_eq!(table.size(&Scope::Global), 8);
            assert_eq!(table.count_all(), 1);
            assert_eq!(table.count(&Scope::Global), 1);
            assert_eq!(
                table.search(&Scope::Global, &"a".to_string()),
                Some(Symbol {
                    scope: Scope::Global,
                    var: "a".to_string(),
                    t: Type::Int,
                    strt: Structure::Identifier,
                    pos: 1,
                    offset: 0,
                    size: 8,
                })
            );
        }
        {
            let mut table = SymbolTable::new();
            table.register_sym(Symbol::new(
                Scope::Local("test".to_string()),
                "a".to_string(),
                Type::Int,
                Structure::Identifier,
            ));
            table.register_sym(Symbol::new(
                Scope::Local("test".to_string()),
                "b".to_string(),
                Type::Int,
                Structure::Identifier,
            ));

            // 期待値
            assert_eq!(table.size(&Scope::Local("test".to_string())), 16);
            assert_eq!(table.count_all(), 2);
            assert_eq!(table.count(&Scope::Local("test".to_string())), 2);
            assert_eq!(
                table.search(&Scope::Local("test".to_string()), &"a".to_string()),
                Some(Symbol {
                    scope: Scope::Local("test".to_string()),
                    var: "a".to_string(),
                    t: Type::Int,
                    strt: Structure::Identifier,
                    pos: 1,
                    offset: 0,
                    size: 8,
                })
            );
            assert_eq!(
                table.search(&Scope::Local("test".to_string()), &"b".to_string()),
                Some(Symbol {
                    scope: Scope::Local("test".to_string()),
                    var: "b".to_string(),
                    t: Type::Int,
                    strt: Structure::Identifier,
                    pos: 2,
                    offset: 8,
                    size: 8,
                })
            );
        }
        {
            let mut table = SymbolTable::new();
            table.register_sym(Symbol::new(
                Scope::Local("test".to_string()),
                "a".to_string(),
                Type::Int,
                Structure::Identifier,
            ));
            table.register_sym(Symbol::new(
                Scope::Local("test".to_string()),
                "b".to_string(),
                Type::Char,
                Structure::Identifier,
            ));

            // 期待値
            assert_eq!(table.size(&Scope::Local("test".to_string())), 9);
            assert_eq!(table.count_all(), 2);
            assert_eq!(table.count(&Scope::Local("test".to_string())), 2);
            assert_eq!(
                table.search(&Scope::Local("test".to_string()), &"a".to_string()),
                Some(Symbol {
                    scope: Scope::Local("test".to_string()),
                    var: "a".to_string(),
                    t: Type::Int,
                    strt: Structure::Identifier,
                    pos: 1,
                    offset: 0,
                    size: 8,
                })
            );
            assert_eq!(
                table.search(&Scope::Local("test".to_string()), &"b".to_string()),
                Some(Symbol {
                    scope: Scope::Local("test".to_string()),
                    var: "b".to_string(),
                    t: Type::Char,
                    strt: Structure::Identifier,
                    pos: 2,
                    offset: 8,
                    size: 1,
                })
            );
        }
        {
            let mut table = SymbolTable::new();
            table.register_sym(Symbol::new(
                Scope::Global,
                "a".to_string(),
                Type::Int,
                Structure::Array(vec![10]),
            ));

            // 期待値
            assert_eq!(table.size(&Scope::Global), 80);
            assert_eq!(table.count_all(), 1);
            assert_eq!(table.count(&Scope::Global), 1);
            assert_eq!(
                table.search(&Scope::Global, &"a".to_string()),
                Some(Symbol {
                    scope: Scope::Global,
                    var: "a".to_string(),
                    t: Type::Int,
                    strt: Structure::Array(vec![10]),
                    pos: 1,
                    offset: 0,
                    size: 80,
                })
            );
        }
        {
            let mut table = SymbolTable::new();
            table.register_sym(Symbol::new(
                Scope::Global,
                "a".to_string(),
                Type::Char,
                Structure::Array(vec![10]),
            ));

            // 期待値
            assert_eq!(table.size(&Scope::Global), 80);
            assert_eq!(table.count_all(), 1);
            assert_eq!(table.count(&Scope::Global), 1);
            assert_eq!(
                table.search(&Scope::Global, &"a".to_string()),
                Some(Symbol {
                    scope: Scope::Global,
                    var: "a".to_string(),
                    t: Type::Char,
                    strt: Structure::Array(vec![10]),
                    pos: 1,
                    offset: 0,
                    size: 80,
                })
            );
        }
        {
            let mut table = SymbolTable::new();
            table.register_sym(Symbol::new(
                Scope::Local("test".to_string()),
                "a".to_string(),
                Type::Char,
                Structure::Pointer,
            ));

            // 期待値
            assert_eq!(table.count_all(), 1);
            assert_eq!(table.size(&Scope::Local("test".to_string())), 8);
            assert_eq!(table.count(&Scope::Local("test".to_string())), 1);
            assert_eq!(
                table.search(&Scope::Local("test".to_string()), &"a".to_string()),
                Some(Symbol {
                    scope: Scope::Local("test".to_string()),
                    var: "a".to_string(),
                    t: Type::Char,
                    strt: Structure::Pointer,
                    pos: 1,
                    offset: 0,
                    size: 8,
                })
            );
        }
        {
            let mut table = SymbolTable::new();
            table.register_sym(Symbol::new(
                Scope::Local("test".to_string()),
                "a".to_string(),
                Type::Char,
                Structure::Identifier,
            ));
            table.register_sym(Symbol::new(
                Scope::Global,
                "a".to_string(),
                Type::Int,
                Structure::Identifier,
            ));

            // 期待値
            assert_eq!(table.count_all(), 2);
            assert_eq!(table.count(&Scope::Global), 1);
            assert_eq!(table.size(&Scope::Global), 8);
            assert_eq!(table.count(&Scope::Local("test".to_string())), 1);
            assert_eq!(table.size(&Scope::Local("test".to_string())), 1);
            assert_eq!(
                table.search(&Scope::Global, &"a".to_string()),
                Some(Symbol {
                    scope: Scope::Global,
                    var: "a".to_string(),
                    t: Type::Int,
                    strt: Structure::Identifier,
                    pos: 1,
                    offset: 0,
                    size: 8,
                })
            );
            assert_eq!(
                table.search(&Scope::Local("test".to_string()), &"a".to_string()),
                Some(Symbol {
                    scope: Scope::Local("test".to_string()),
                    var: "a".to_string(),
                    t: Type::Char,
                    strt: Structure::Identifier,
                    pos: 1,
                    offset: 0,
                    size: 1,
                })
            );
        }
    }
}
