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
    Struct,
    Unknown(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Structure {
    Identifier,
    Pointer,
    Array(Vec<usize>),
    Type,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub scope: Scope,         // スコープ
    pub var: String,          // 変数名
    pub t: Type,              // 型
    pub strt: Structure,      // 構造
    pub pos: usize,           // ポジション
    pub offset: usize,        // オフセット
    pub size: usize,          // サイズ
    pub members: Vec<Symbol>, // メンバー変数
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    table: Vec<Symbol>,
}

impl Symbol {
    // コンストラクタ
    #[allow(dead_code)]
    pub fn new(s: Scope, v: String, ty: Type, st: Structure) -> Self {
        Symbol {
            scope: s,
            var: v,
            t: ty,
            strt: st,
            pos: 0,
            offset: 0,
            size: 0,
            members: vec![],
        }
    }

    /// メンバー登録
    pub fn regist_mem(&mut self, mem: Vec<Symbol>) {
        // サイズを設定したメンバーを保存
        for mut m in mem {
            m.size = match m.t {
                Type::Int => 4,
                Type::Char => 1,
                _ => 0,
            };
            self.members.push(m);
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
        let mut reg = sym;
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
                    Structure::Array(ref v) => self.type_size(&sym) * v.iter().product::<usize>(),
                    _ => self.type_size(&sym)
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
                        let count: usize = v.iter().product();
                        reg.pos = pre_sym.pos + count;
                        reg.size = self.type_size(&sym) * count;
                        reg.offset = pre_sym.offset + self.type_size(&pre_sym) * count;
                        reg.offset = (reg.offset / 8) * 8;
                        self.table.push(reg);
                    }
                    Structure::Pointer => {
                        reg.pos = pre_sym.pos + 1;
                        reg.size = self.type_size(&sym);
                        reg.offset = pre_sym.offset + self.type_size(&pre_sym);
                        reg.offset = (reg.offset / 8) * 8;
                        self.table.push(reg);
                    }
                    _ => {
                        reg.pos = pre_sym.pos + 1;
                        reg.size = self.type_size(&sym);
                        reg.offset = pre_sym.offset + self.type_size(&pre_sym);
                        reg.offset = (reg.offset / 8) * 8 + 8;
                        self.table.push(reg);
                    }
                }
            }
        };
    }

    // シンボルサーチ
    #[allow(dead_code)]
    pub fn search(&self, scope: &Scope, var: &str) -> Option<Symbol> {
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
            .count()
    }

    // 型に応じたサイズ取得
    fn type_size(&self, sym: &Symbol) -> usize {
        match sym.strt {
            Structure::Pointer => 8,
            Structure::Array(_) => 8,
            _ => {
                match sym.t {
                    Type::Int => 4,
                    Type::Char => 1,
                    Type::Struct => {
                        // メンバーのサイズを加算し、返す
                        if let Some(max_mem) = sym.members.iter().max_by(|a, b| a.size.cmp(&b.size)) {
                            // 各メンバーサイズを加算
                            let size = sym.members.iter().fold(0, |acc, s| acc + self.type_size(s));

                            // アライメントを考慮したサイズを返す
                            if size % max_mem.size == 0 { size }
                            else { (size / max_mem.size) * max_mem.size + max_mem.size }
                        }
                        else {
                            0
                        }
                    }
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
                Structure::Identifier => acc + self.type_size(&sym),
                // 配列の場合、要素数を考慮
                Structure::Array(ref items) => {
                    acc + items.iter().fold(0, |acc2, i| acc2 + (i * self.type_size(&sym)))
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
            assert_eq!(table.size(&Scope::Global), 4);
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
                    size: 4,
                    members: vec![],
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
            assert_eq!(table.size(&Scope::Local("test".to_string())), 8);
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
                    size: 4,
                    members: vec![],
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
                    size: 4,
                    members: vec![],
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
            assert_eq!(table.size(&Scope::Local("test".to_string())), 5);
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
                    size: 4,
                    members: vec![],
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
                    members: vec![],
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
                    members: vec![],
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
                    members: vec![],
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
                    members: vec![],
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
            assert_eq!(table.size(&Scope::Global), 4);
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
                    size: 4,
                    members: vec![],
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
                    members: vec![],
                })
            );
        }
    }

    #[test]
    fn test_type_size() {
        let table = SymbolTable::new();
        {
            assert_eq!(
                1,
                table.type_size(
                    &Symbol {
                        scope: Scope::Local("test".to_string()),
                        var: "a".to_string(),
                        t: Type::Char,
                        strt: Structure::Identifier,
                        pos: 0,
                        offset: 0,
                        size: 1,
                        members: vec![],
                    }
                )
            )
        }
        {
            assert_eq!(
                4,
                table.type_size(
                    &Symbol {
                        scope: Scope::Local("test".to_string()),
                        var: "a".to_string(),
                        t: Type::Int,
                        strt: Structure::Identifier,
                        pos: 0,
                        offset: 0,
                        size: 4,
                        members: vec![],
                    }
                )
            )
        }
        {
            assert_eq!(
                1,
                table.type_size(
                    &Symbol {
                        scope: Scope::Local("test".to_string()),
                        var: "a".to_string(),
                        t: Type::Struct,
                        strt: Structure::Type,
                        pos: 0,
                        offset: 0,
                        size: 0,
                        members: vec![
                            Symbol {
                                scope: Scope::Local("test".to_string()),
                                var: "a".to_string(),
                                t: Type::Char,
                                strt: Structure::Identifier,
                                pos: 0,
                                offset: 0,
                                size: 1,
                                members: vec![],
                            }
                        ],
                    }
                )
            )
        }
        {
            assert_eq!(
                2,
                table.type_size(
                    &Symbol {
                        scope: Scope::Local("test".to_string()),
                        var: "a".to_string(),
                        t: Type::Struct,
                        strt: Structure::Type,
                        pos: 0,
                        offset: 0,
                        size: 0,
                        members: vec![
                            Symbol {
                                scope: Scope::Local("test".to_string()),
                                var: "a".to_string(),
                                t: Type::Char,
                                strt: Structure::Identifier,
                                pos: 0,
                                offset: 0,
                                size: 1,
                                members: vec![],
                            },
                            Symbol {
                                scope: Scope::Local("test".to_string()),
                                var: "a".to_string(),
                                t: Type::Char,
                                strt: Structure::Identifier,
                                pos: 0,
                                offset: 0,
                                size: 1,
                                members: vec![],
                            }
                        ],
                    }
                )
            )
        }
        {
            assert_eq!(
                8,
                table.type_size(
                    &Symbol {
                        scope: Scope::Local("test".to_string()),
                        var: "a".to_string(),
                        t: Type::Struct,
                        strt: Structure::Type,
                        pos: 0,
                        offset: 0,
                        size: 0,
                        members: vec![
                            Symbol {
                                scope: Scope::Local("test".to_string()),
                                var: "a".to_string(),
                                t: Type::Char,
                                strt: Structure::Identifier,
                                pos: 0,
                                offset: 0,
                                size: 1,
                                members: vec![],
                            },
                            Symbol {
                                scope: Scope::Local("test".to_string()),
                                var: "a".to_string(),
                                t: Type::Int,
                                strt: Structure::Identifier,
                                pos: 0,
                                offset: 0,
                                size: 4,
                                members: vec![],
                            }
                        ],
                    }
                )
            )
        }
    }
}
