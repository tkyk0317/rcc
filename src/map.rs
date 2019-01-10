#[doc = "マップ構造"]
use std::collections::HashMap;

#[derive(Debug)]
pub struct Map<T: Clone> {
    count: usize,
    map: HashMap<String, T>,
}

impl<T: Clone> Map<T> {
    // コンストラクタ.
    pub fn new() -> Map<T> {
        Map {
            count: 0,
            map: HashMap::new(),
        }
    }

    // 要素追加.
    pub fn add(&mut self, k: String, v: T) {
        if false == self.map.contains_key(&k) {
            self.count += 1;
        }
        self.map.insert(k, v);
    }

    // 検索.
    pub fn search(&self, k: &String) -> Option<&T> {
        self.map.get(k).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        {
            // データが存在しない.
            let mut m = Map::<String>::new();
            let k = "key".to_string();
            let v = "value".to_string();
            m.add(k, v);

            assert_eq!(m.count, 1)
        }
        {
            // データが存在.
            let mut m = Map::<String>::new();
            let k = "key".to_string();
            let v = "value".to_string();
            m.add(k.clone(), v.clone());
            m.add(k.clone(), v.clone());

            assert_eq!(m.count, 1)
        }
        {
            // データが満タン.
            let mut m = Map::<String>::new();
            let v = "value".to_string();
            for i in 0..1024 {
                m.add(i.to_string(), v.clone());
            }
            m.add("t".to_string(), v.clone());

            assert_eq!(m.count, 1025)
        }
    }
}
