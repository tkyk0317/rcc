#[doc = "シンボルテーブル"]

use map::Map;

#[derive(Debug)]
pub struct SymbolTable {
    map: Map<String>,
}

impl SymbolTable {

    /**
     * コンストラクタ.
     */
    pub fn new() -> Self {
        SymbolTable {
            map: Map::new()
        }
    }

    /**
     * シンボル追加.
     */
    pub fn push(&mut self, k: String, v: String) -> bool {
        self.map.add(k ,v)
    }

    /**
     * シンボル検索
     */
    pub fn search(&self, k: &String) -> Option<String> {
        self.map.search(k)
    }
}
