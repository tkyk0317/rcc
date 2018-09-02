/**
 * トークン
 */

// トークン識別子.
#[derive(Debug,Clone,PartialEq)]
pub enum Token {
    Number,           // 数値.
    Variable,         // 変数.
    Equal,            // イコール演算子.
    Plus,             // プラス演算子.
    Minus,            // マイナス演算子.
    LeftBracket,      // 左括弧.
    RightBracket,     // 右括弧.
    Unknown,          // 不明.
}

// トークンデータ.
#[derive(Debug,Clone)]
pub struct TokenInfo {
    token: Token,  // トークン種別.
    val: String,   // 内容.
}

// トークン実装.
impl TokenInfo {
    // コンストラクタ.
    pub fn new(token: Token, val: String) -> TokenInfo {
        TokenInfo { token: token, val: val }
    }

    // トークン種別取得.
    pub fn get_token_type(&self) -> Token {
        self.token.clone()
    }
}

