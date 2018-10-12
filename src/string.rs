#[doc = "文字列構造"]
pub struct StringData {
    len: usize, // 文字列長.
    size: usize, // 文字列バッファサイズ.
    buf: Vec<char>, // 文字列領域
}

impl StringData {
    // コンストラクタ.
    pub fn new() -> StringData {
        let l = 64;
        StringData {
            len: 0,
            size: 64,
            buf: Vec::<char>::with_capacity(l),
        }
    }

    // 文字列設定.
    pub fn push(&mut self, c: char) {
        self.buf.push(c);
        self.len = self.len + 1;
        if self.len >= self.size {
            // メモリを再確保.
            let s = self.buf.clone();
            self.buf = Vec::<char>::with_capacity(self.size * 2);
            self.buf = s;
            self.size = self.size * 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        {
            // 生成したばかりのインスタンステスト
            let s = StringData::new();
            assert_eq!(s.len, 0);
            assert_eq!(s.size, 64);
        }
        {
            // 一文字追加
            let mut s = StringData::new();
            s.push('a');

            assert_eq!(s.len, 1);
            assert_eq!(s.size, 64);
            assert_eq!(s.buf[0], 'a');
        }
        {
            // 64文字追加.
            let mut s = StringData::new();
            for _ in 0..64 {
                s.push('9');
            }

            assert_eq!(s.len, 64);
            assert_eq!(s.size, 128);
            assert_eq!(s.buf[0], '9');
            assert_eq!(s.buf[63], '9');
        }
        {
            // 65文字追加.
            let mut s = StringData::new();
            for _ in 0..64 {
                s.push('9');
            }
            s.push('1');

            assert_eq!(s.len, 65);
            assert_eq!(s.size, 128);
            assert_eq!(s.buf[0], '9');
            assert_eq!(s.buf[63], '9');
            assert_eq!(s.buf[64], '1');
        }
    }
}
