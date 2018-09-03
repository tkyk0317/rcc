use token::TokenInfo;
use token::Token;

/**
 * 字句解析.
 */
pub struct LexicalAnalysis<'a> {
    input: &'a String,
    pos: usize,
    tokens: Vec<TokenInfo>,
}

impl<'a> LexicalAnalysis<'a> {
    // コンストラクタ.
    pub fn new(input: &'a String) -> LexicalAnalysis {
        LexicalAnalysis { input: input, pos: 0, tokens: vec![] }
    }

    // トークン群取得.
    pub fn get_tokens(&self) -> &Vec<TokenInfo> {
        &self.tokens
    }

    // トークン読み込み.
    pub fn read_token(&mut self) {
        // 終了まで読み込み、字句解析を行う.
        while false == self.is_eof() {
            // 空白部分は読み飛ばし.
            self.skip_space();

            // 一文字読み取って、トークン生成.
            match self.next() {
                Some(v) => {
                    let mut token;
                    match v {
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            // 数値が続く部分まで抜き出し、トークン生成.
                            token = self.generate_number_token(v);
                        }
                        s if true == s.is_alphabetic() => {
                            // アルファベットか数値が続くまで抜き出し、トークン生成.
                            token = self.generate_variable_token(v);
                        }
                        '=' => { token = TokenInfo::new(Token::Equal, v.to_string()); }
                        '+' => { token = TokenInfo::new(Token::Plus, v.to_string()); }
                        '-' => { token = TokenInfo::new(Token::Minus, v.to_string()); }
                        '*' => { token = TokenInfo::new(Token::Multi, v.to_string()); }
                        '(' => { token = TokenInfo::new(Token::LeftBracket, v.to_string()); }
                        ')' => { token = TokenInfo::new(Token::RightBracket, v.to_string()); }
                        _ => { token = TokenInfo::new(Token::Unknown, v.to_string()); }
                    }
                    self.tokens.push(token);
                }
                _ => {}
            }
        }
    }

    // 文字を読み出す.
    fn read(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    // 文字を読み出して次へ進める.
    fn next(&mut self) -> Option<char> {
        if true == self.is_eof() {
            None
        }
        else {
            let s = self.input.chars().nth(self.pos);
            self.pos = self.pos + 1;
            s
        }
    }

    // 文字列終端チェック.
    fn is_eof(&self) -> bool {
        self.pos > (self.input.len() - 1)
    }

    // 空白読み飛ばし.
    fn skip_space(&mut self) {
        while false == self.is_eof() &&
              true == self.input.chars().nth(self.pos).unwrap().is_whitespace() {
            self.next();
        }
    }

    // 数値トークン生成.
    fn generate_number_token(&mut self, v: char) -> TokenInfo {
        let mut s = String::new();
        s.push(v);

        while false == self.is_eof() && true == self.read().unwrap().is_digit(10) {
            s.push(self.next().unwrap());
        }
        TokenInfo::new(Token::Number, s)
    }

    // 変数トークン生成.
    fn generate_variable_token(&mut self, v: char) -> TokenInfo {
        let mut s = String::new();
        s.push(v);

        while false == self.is_eof() &&
              (true == self.read().unwrap().is_alphabetic() || true == self.read().unwrap().is_digit(10)) {
            s.push(self.next().unwrap());
        }
        TokenInfo::new(Token::Variable, s)
    }
}

