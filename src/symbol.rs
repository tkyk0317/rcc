#[doc = "シンボルテーブル"]

use map::Map;

#[derive(Debug)]
pub struct Symbol {
    map: Map<String>,
}

impl Symbol {

    /**
     * コンストラクタ.
     */
    pub fn new() -> Self {
        Symbol {
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
