use ast::{Structure, Type};
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
    pub t: Type,
    pub s: Structure,
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
    pub fn push(&mut self, k: String, t: &Type, s: &Structure) -> bool {
        let res = self.map.add(
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
            Structure::Array(s) => self.count += s.iter().fold(1, |acc, i| acc * i),
            _ => self.count += 1,
        };
        res
    }

    #[doc = "シンボル検索"]
    pub fn search(&self, k: &String) -> Option<Meta> {
        self.map.search(k)
    }

    #[doc = "シンボル数取得"]
    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        {
            let mut s = SymbolTable::new(Scope::Local);
            s.push("key".to_string(), &Type::Int, &Structure::Identifier);
            assert_eq!(s.count(), 1);
            assert_eq!(
                s.search(&"key".to_string()),
                Some(Meta {
                    scope: Scope::Local,
                    p: 0,
                    t: Type::Int,
                    s: Structure::Identifier
                })
            )
        }
        {
            let mut s = SymbolTable::new(Scope::Global);
            s.push("key".to_string(), &Type::Int, &Structure::Identifier);
            assert_eq!(s.count(), 1);
            assert_eq!(s.search(&"not_exist_key".to_string()), None)
        }
    }
}
