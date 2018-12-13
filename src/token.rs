#[doc = "トークン"]
// トークン識別子.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    If,               // if文.
    Else,             // else文.
    Do,               // do-while文.
    While,            // while文.
    For,              // For文.
    Continue,         // continue文.
    Break,            // break文.
    Return,           // return文.
    BitReverse,       // ビット反転演算子.
    And,              // &演算子
    BitOr,            // ビットOR演算子
    BitXor,           // ビットXOR演算子
    LeftShift,        // 左シフト演算子.
    RightShift,       // 右シフト演算子.
    Question,         // ?演算子.
    Colon,            // コロン.
    Comma,            // カンマ.
    LogicalAnd,       // &&演算子.
    LogicalOr,        // ||演算子.
    Equal,            // 等価演算子.
    NotEqual,         // 否等価演算子.
    LessThan,         // 比較演算子(<).
    GreaterThan,      // 比較演算子(>).
    LessThanEqual,    // 比較演算子(<=).
    GreaterThanEqual, // 比較演算子(>=).
    Number,           // 数値.
    Variable,         // 変数.
    Plus,             // プラス演算子.
    Minus,            // マイナス演算子.
    Multi,            // 乗算演算子.
    Division,         // 除算演算子.
    Remainder,        // 余り演算子.
    LeftParen,        // 左括弧.
    RightParen,       // 右括弧.
    LeftBrace,        // 左波括弧.
    RightBrace,       // 右波括弧.
    Not,              // 否定演算子.
    SemiColon,        // セミコロン.
    Assign,           // 代入演算子.
    Int,              // int型.
    IntPointer,       // intポインタ
    End,              // 終了.
}

// 位置情報
#[derive(Debug, Clone, PartialEq)]
pub struct PosInfo {
    name: String,
    row: usize,
    pub col: usize,
}

impl PosInfo {
    pub fn new(name: String, row: usize, col: usize) -> Self {
        PosInfo {
            name: name,
            row: row,
            col: col,
        }
    }
}

// トークンデータ.
#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    token: Token,     // トークン種別.
    val: String,      // 内容.
    pub pos: PosInfo, // 位置情報.
}

// トークン実装.
impl TokenInfo {
    // コンストラクタ.
    pub fn new(token: Token, val: String, pos: (String, usize, usize)) -> TokenInfo {
        TokenInfo {
            token: token,
            val: val,
            pos: PosInfo::new(pos.0, pos.1, pos.2),
        }
    }

    // トークン種別取得.
    pub fn get_token_type(&self) -> Token {
        self.token.clone()
    }

    // トークン値取得.
    pub fn get_token_value(&self) -> String {
        self.val.clone()
    }
}
