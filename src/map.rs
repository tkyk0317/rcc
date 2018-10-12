#[doc = "マップ構造"]
#[derive(Debug)]
pub struct Map<T: Clone> {
    count: usize,
    keys: Vec<String>,
    values: Vec<T>,
}

impl<T: Clone> Map<T> {
    // コンストラクタ.
    pub fn new() -> Map<T> {
        Map {
            count: 0,
            keys: Vec::<String>::with_capacity(1024),
            values: Vec::<T>::with_capacity(1024),
        }
    }

    // 要素追加.
    pub fn add(&mut self, k: String, v: T) -> bool {
        // 同じキーがある場合、上書き.
        match self.search(&k) {
            Some(_) => true,
            None => {
                // 存在しない場合は、追加.
                if self.count >= 1024 {
                    return false;
                } else {
                    self.keys.push(k.clone());
                    self.values.push(v.clone());
                    self.count = self.count + 1;
                    return true;
                }

            }
        }
    }

    // 検索.
    pub fn search(&self, k: &String) -> Option<T> {
        for x in 0..self.count {
            if *k == self.keys[x] {
                return Some(self.values[x].clone());
            }
        }
        None
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
            let ret = m.add(k, v);

            assert_eq!(ret, true);
            assert_eq!(m.count, 1)
        }
        {
            // データが存在.
            let mut m = Map::<String>::new();
            let k = "key".to_string();
            let v = "value".to_string();
            let _ = m.add(k.clone(), v.clone());
            let ret = m.add(k.clone(), v.clone());

            assert_eq!(ret, true);
            assert_eq!(m.count, 1)
        }
        {
            // データが満タン.
            let mut m = Map::<String>::new();
            let v = "value".to_string();
            for i in 0..1024 {
                let _ = m.add(i.to_string(), v.clone());
            }
            let ret = m.add("t".to_string(), v.clone());

            assert_eq!(ret, false);
            assert_eq!(m.count, 1024)
        }
    }
}
