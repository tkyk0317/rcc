#[doc = "シンボルテーブル"]
use map::Map;
use ast::Type;

#[derive(Debug)]
pub struct SymbolTable {
    count: usize,
    map: Map<Meta>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Meta {
    pub p: usize,
    pub t: Type,
}

impl SymbolTable {
    #[doc = "コンストラクタ"]
    pub fn new() -> Self {
        SymbolTable {
            count: 0,
            map: Map::new(),
        }
    }

    #[doc = "シンボル追加"]
    pub fn push(&mut self, k: String, t: &Type) -> bool {
        let res = self.map.add(
            k,
            Meta {
                p: self.count,
                t: t.clone(),
            },
        );
        self.count += 1;
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
            let mut s = SymbolTable::new();
            s.push("key".to_string(), &Type::Int);
            assert_eq!(s.count(), 1);
            assert_eq!(
                s.search(&"key".to_string()),
                Some(Meta {
                    p: 0,
                    t: Type::Int,
                })
            )
        }
        {
            let mut s = SymbolTable::new();
            s.push("key".to_string(), &Type::Int);
            assert_eq!(s.count(), 1);
            assert_eq!(s.search(&"not_exist_key".to_string()), None)
        }
    }
}
